//! `hp_energy` against a verbatim copy of the previous code, which read the
//! last lattice position back with `expect`.

use super::{hp_energy, HPResidue, LatticeMove};

fn old_hp_energy(sequence: &[HPResidue], moves: &[LatticeMove]) -> i32 {
    if sequence.is_empty() || moves.len() + 1 != sequence.len() {
        return 0;
    }
    let mut positions: Vec<(i32, i32)> = Vec::with_capacity(sequence.len());
    positions.push((0, 0));
    for &mv in moves {
        let (x, y) = *positions.last().expect("starts with the origin");
        let next = match mv {
            LatticeMove::Up => (x, y + 1),
            LatticeMove::Down => (x, y - 1),
            LatticeMove::Left => (x - 1, y),
            LatticeMove::Right => (x + 1, y),
        };
        positions.push(next);
    }
    let mut energy = 0i32;
    let n = positions.len();
    for i in 0..n {
        for j in (i + 2)..n {
            if sequence[i] == HPResidue::H && sequence[j] == HPResidue::H {
                let (xi, yi) = positions[i];
                let (xj, yj) = positions[j];
                let manhattan = (xi - xj).abs() + (yi - yj).abs();
                if manhattan == 1 {
                    energy -= 1;
                }
            }
        }
    }
    energy
}

#[test]
fn energies_are_unchanged() {
    let moves = [
        LatticeMove::Up,
        LatticeMove::Down,
        LatticeMove::Left,
        LatticeMove::Right,
    ];
    let mut contacts = 0usize;
    // Every sequence of up to 6 residues with every conformation, plus
    // mismatched lengths.
    for len in 0..7usize {
        for seq_bits in 0..(1u32 << len) {
            let sequence: Vec<HPResidue> = (0..len)
                .map(|i| {
                    if seq_bits >> i & 1 == 1 {
                        HPResidue::H
                    } else {
                        HPResidue::P
                    }
                })
                .collect();
            let n_moves = len.saturating_sub(1);
            for code in 0..4usize.pow(n_moves as u32) {
                let conf: Vec<LatticeMove> = (0..n_moves)
                    .map(|k| moves[(code / 4usize.pow(k as u32)) % 4])
                    .collect();
                let e = hp_energy(&sequence, &conf);
                assert_eq!(e, old_hp_energy(&sequence, &conf));
                if e < 0 {
                    contacts += 1;
                }
            }
            assert_eq!(hp_energy(&sequence, &[LatticeMove::Up; 7]), 0);
        }
    }
    assert!(contacts > 0);
}
