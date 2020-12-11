fn main() {
    match rustc_version::version_meta()
        .expect("query rustc release channel")
        .channel
    {
        rustc_version::Channel::Nightly => {
            println!("cargo:rustc-cfg=nightly");
        }
        _ => {}
    }
}
