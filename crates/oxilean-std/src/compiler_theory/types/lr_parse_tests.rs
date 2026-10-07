//! `LRParser::parse` against a verbatim copy of the previous code, which
//! read the top of the stack with `expect`.

use super::LRParser;

fn old_parse(p: &LRParser, input: &[usize]) -> Result<bool, String> {
    let mut stack: Vec<usize> = vec![0];
    let mut pos = 0usize;
    loop {
        let state = *stack.last().expect("stack is non-empty");
        let tok = input.get(pos).copied();
        let tok_idx = tok.unwrap_or(usize::MAX);
        let action = if tok_idx < p.action_table[state].len() {
            &p.action_table[state][tok_idx]
        } else {
            ""
        };
        if action == "acc" {
            return Ok(true);
        } else if let Some(stripped_s) = action.strip_prefix('s') {
            let next: usize = stripped_s.parse().map_err(|_| "bad shift".to_string())?;
            stack.push(tok_idx);
            stack.push(next);
            pos += 1;
        } else if let Some(stripped_r) = action.strip_prefix('r') {
            let rule: usize = stripped_r.parse().map_err(|_| "bad reduce".to_string())?;
            let pop = 2 * rule;
            if stack.len() < pop {
                return Err("stack underflow".to_string());
            }
            stack.truncate(stack.len() - pop);
            let top = *stack.last().expect("stack is non-empty");
            let nt_idx = rule;
            let goto_state = if nt_idx < p.goto_table[top].len() {
                p.goto_table[top][nt_idx]
            } else {
                return Err("goto error".to_string());
            };
            stack.push(nt_idx);
            stack.push(goto_state);
        } else {
            return Err(format!("parse error at token {:?}", tok));
        }
    }
}

fn table(rows: &[&[&str]]) -> Vec<Vec<String>> {
    rows.iter()
        .map(|row| row.iter().map(|s| s.to_string()).collect())
        .collect()
}

#[test]
fn a_small_grammar_parses_as_before() {
    // S -> a, with the end marker as token 1.
    let p = LRParser::new(
        3,
        table(&[&["s1", ""], &["", "r1"], &["", "acc"]]),
        vec![vec![0, 2], vec![], vec![]],
    );
    for input in [
        vec![0, 1],
        vec![1],
        vec![0],
        vec![],
        vec![0, 0, 1],
        vec![2, 1],
    ] {
        assert_eq!(p.parse(&input), old_parse(&p, &input), "{input:?}");
    }
    assert_eq!(p.parse(&[0, 1]), Ok(true));
}

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.0 = x;
        x
    }
    fn below(&mut self, n: u64) -> u64 {
        self.next() % n
    }
}

#[test]
fn random_tables_give_the_same_answers_and_errors() {
    let mut rng = Rng(0x0f1e_2d3c_4b5a_6978);
    let mut accepted = 0usize;
    let mut underflows = 0usize;
    for _ in 0..4000 {
        let states = 1 + rng.below(5) as usize;
        let terminals = 1 + rng.below(3) as usize;
        // Every reduction pops at least two pairs and every shift consumes a
        // token, so each run ends; shift and goto targets are valid states.
        let action_table: Vec<Vec<String>> = (0..states)
            .map(|_| {
                (0..terminals)
                    .map(|_| match rng.below(8) {
                        0 | 1 => format!("s{}", rng.below(states as u64)),
                        2 | 3 => format!("r{}", 2 + rng.below(2)),
                        4 => "acc".to_string(),
                        5 => "sx".to_string(),
                        6 => "r".to_string(),
                        _ => String::new(),
                    })
                    .collect()
            })
            .collect();
        let goto_table: Vec<Vec<usize>> = (0..states)
            .map(|_| {
                let len = rng.below(5) as usize;
                (0..len)
                    .map(|_| rng.below(states as u64) as usize)
                    .collect()
            })
            .collect();
        let p = LRParser::new(states, action_table, goto_table);
        let input: Vec<usize> = (0..rng.below(7))
            .map(|_| rng.below(terminals as u64 + 1) as usize)
            .collect();
        let new = p.parse(&input);
        assert_eq!(new, old_parse(&p, &input), "{p:?} on {input:?}");
        match new {
            Ok(_) => accepted += 1,
            Err(e) if e == "stack underflow" => underflows += 1,
            Err(_) => {}
        }
    }
    assert!(accepted > 0 && underflows > 0);
}
