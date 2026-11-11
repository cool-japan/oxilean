//! `DecisionTable::extract_rules` against a verbatim copy of the previous
//! code, which took an arbitrary member of each class with `expect`.

use super::{DecisionTable, InformationSystem};
use std::collections::{HashMap, HashSet};

type Rule = (Vec<(usize, u32)>, u32);

fn old_extract_rules(table: &DecisionTable) -> Vec<(HashMap<usize, u32>, u32)> {
    let cond = table.condition_attrs();
    let mut rules = Vec::new();
    let classes = table.info.indiscernibility_classes(&cond);
    for cls in &classes {
        let obj = *cls.iter().next().expect("a class has a member");
        let cond_map: HashMap<usize, u32> =
            cond.iter().map(|&a| (a, table.info.get(obj, a))).collect();
        let decision = table.info.get(obj, table.decision_attr);
        rules.push((cond_map, decision));
    }
    rules
}
fn normalized(rules: Vec<(HashMap<usize, u32>, u32)>) -> Vec<Rule> {
    let mut out: Vec<Rule> = rules
        .into_iter()
        .map(|(m, d)| {
            let mut v: Vec<(usize, u32)> = m.into_iter().collect();
            v.sort_unstable();
            (v, d)
        })
        .collect();
    out.sort();
    out
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
fn rules_equal_the_previous_ones_and_name_a_member_of_each_class() {
    let mut rng = Rng(0x1357_9bdf_2468_ace0);
    let mut inconsistent = 0usize;
    for _ in 0..600 {
        let n_objects = rng.below(9) as usize;
        let n_cond = 1 + rng.below(3) as usize;
        let mut info = InformationSystem::new(n_objects, n_cond + 1);
        for obj in 0..n_objects {
            for attr in 0..n_cond {
                info.set(obj, attr, rng.below(3) as u32);
            }
        }
        // A consistent table first: the decision is a function of the conditions.
        for obj in 0..n_objects {
            let d = (0..n_cond).map(|a| info.get(obj, a)).sum::<u32>() % 2;
            info.set(obj, n_cond, d);
        }
        let table = DecisionTable::new(info.clone(), n_cond);
        assert_eq!(
            normalized(table.extract_rules()),
            normalized(old_extract_rules(&table))
        );
        let cond = table.condition_attrs();
        let seeded: Vec<HashSet<usize>> = table
            .info
            .seeded_indiscernibility_classes(&cond)
            .into_iter()
            .map(|(seed, cls)| {
                assert!(cls.contains(&seed));
                cls
            })
            .collect();
        assert_eq!(seeded, table.info.indiscernibility_classes(&cond));
        // Then any decisions: each rule's decision is that of a member of its class.
        for obj in 0..n_objects {
            info.set(obj, n_cond, rng.below(2) as u32);
        }
        let table = DecisionTable::new(info, n_cond);
        let classes = table.info.indiscernibility_classes(&cond);
        let rules = table.extract_rules();
        assert_eq!(rules.len(), classes.len());
        for ((cond_map, decision), cls) in rules.iter().zip(&classes) {
            assert!(cls.iter().any(|&o| {
                table.info.get(o, n_cond) == *decision
                    && cond
                        .iter()
                        .all(|&a| cond_map.get(&a) == Some(&table.info.get(o, a)))
            }));
            let decisions: HashSet<u32> = cls.iter().map(|&o| table.info.get(o, n_cond)).collect();
            if decisions.len() > 1 {
                inconsistent += 1;
            }
        }
    }
    assert!(inconsistent > 0);
}
