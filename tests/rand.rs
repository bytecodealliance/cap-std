#[test]
fn test_std_rng_from_entropy() {
    let rng = cap_rand::std_rng_from_entropy(cap_rand::ambient_authority());
    assert_eq!(rng.clone(), rng);
}
