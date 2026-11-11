//! `SpectralTripleData::resolvent_is_compact` on short and longer spectra.

use super::SpectralTripleData;

#[test]
fn compares_the_last_eigenvalue_with_the_first() {
    let base = || SpectralTripleData::new("A", 4, 0);
    assert!(base().resolvent_is_compact());
    assert!(base().with_eigenvalues(vec![3.0]).resolvent_is_compact());
    assert!(base()
        .with_eigenvalues(vec![1.0, 2.0])
        .resolvent_is_compact());
    assert!(!base()
        .with_eigenvalues(vec![2.0, 2.0])
        .resolvent_is_compact());
    assert!(!base()
        .with_eigenvalues(vec![5.0, 9.0, 1.0])
        .resolvent_is_compact());
    assert!(!base()
        .with_eigenvalues(vec![f64::NAN, 1.0])
        .resolvent_is_compact());
}
