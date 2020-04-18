use std::env;

fn main() {
    let app_name = "fix-gnupg-permissions";
    println!("cargo:rustc-env=APP_NAME={}", app_name);
    println!(
        "cargo:rustc-env=AUTHOR_EMAIL=Billie Thompson <billie+{}@purplebooth.co.uk>",
        app_name
    );
    println!(
        "cargo:rustc-env=VERSION={}",
        env::var("VERSION").unwrap_or_else(|_| "dev".to_string())
    );
}
