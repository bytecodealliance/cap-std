use cap_std::{ambient_authority, fs::File};

#[test]
fn test_open_ambient() {
    let _ = File::open_ambient("Cargo.toml", ambient_authority()).unwrap();
}
