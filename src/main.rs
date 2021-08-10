use clap::App;
use std::{
    env, error, fs,
    os::unix::fs::{OpenOptionsExt, PermissionsExt},
};

const USER_ONLY_OCTLET: u32 = 0o600;
const USER_PLUS_LIST_OCTLET: u32 = 0o700;

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

use clap::{crate_authors, crate_version};

fn main() -> Result<()> {
    App::new(env!("CARGO_PKG_NAME"))
        .version(crate_version!())
        .author(crate_authors!())
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .get_matches();
    fix_gnupg_permissions(gnuhome_dir)
}

fn fix_gnupg_permissions(gnu_home: impl FnOnce() -> String) -> Result<()> {
    let gnupg_dir = gnu_home();
    let config_file = format!("{}/gpg-agent.conf", gnupg_dir);
    fs::create_dir_all(&gnupg_dir)?;
    set_directory_permission(&gnupg_dir, USER_PLUS_LIST_OCTLET);
    set_file_permission(&config_file, USER_ONLY_OCTLET);

    Ok(())
}

fn gnuhome_dir() -> String {
    let home = dirs::home_dir().unwrap();
    let gnupg_home = home.join(".gnupg");

    env::var("GNUPGHOME").unwrap_or_else(|_| gnupg_home.to_str().unwrap().to_string())
}

fn set_file_permission(config_file: &str, permission: u32) {
    fs::OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .mode(permission)
        .open(config_file)
        .unwrap();

    let mut permissions = fs::metadata(config_file).unwrap().permissions();
    permissions.set_mode(permission); // Read/write for owner and read for others.
    fs::set_permissions(config_file, permissions).unwrap();
}

fn set_directory_permission(config_file: &str, permission: u32) {
    let mut permissions = fs::metadata(config_file).unwrap().permissions();
    permissions.set_mode(permission); // Read/write for owner and read for others.
    fs::set_permissions(config_file, permissions).unwrap();
}

#[cfg(test)]
mod tests {
    #![allow(clippy::wildcard_imports)]

    use super::*;

    const PERMISSIONS_ONLY_MASK: u32 = 0o777;

    use std::{fs, fs::File, os::unix::fs::PermissionsExt, process::Command};
    use tempfile::tempdir;

    #[test]
    fn no_gnupg_home_set() {
        let old = env::var("GNUPGHOME");
        env::remove_var("GNUPGHOME");

        let home = dirs::home_dir().unwrap();
        let gnupg_home = home.join(".gnupg");

        assert_eq!(gnupg_home.to_str().unwrap(), gnuhome_dir());

        if let Ok(var) = old {
            env::set_var("GNUPGHOME", var);
        }
    }

    #[test]
    fn it_uses_the_directory_set_in_gnupg_home() {
        let old = env::var("GNUPGHOME");
        env::set_var("GNUPGHOME", "something");

        assert_eq!("something", gnuhome_dir());

        if let Ok(var) = old {
            env::set_var("GNUPGHOME", var);
        } else {
            env::remove_var("GNUPGHOME");
        }
    }

    #[test]
    fn files_that_are_writable_by_others_are_protected() {
        let temp_path = tempdir().unwrap().into_path();
        let file_path = temp_path.join("gpg-agent.conf");
        File::create(&file_path).unwrap();

        let command_output = Command::new("chmod")
            .arg("-R")
            .arg("ag+wrx")
            .arg(&file_path)
            .status()
            .expect("Changing permissions failed?");

        assert_eq!(command_output.code().unwrap(), 0);
        assert!(fix_gnupg_permissions(|| temp_path.to_str().unwrap().to_string()).is_ok());

        let actual = fs::metadata(file_path).unwrap().permissions().mode();

        assert_eq!(
            actual & PERMISSIONS_ONLY_MASK,
            USER_ONLY_OCTLET,
            "Expected {:o} found {:o}",
            USER_ONLY_OCTLET,
            actual & PERMISSIONS_ONLY_MASK
        );
    }

    #[test]
    fn directories_that_are_writable_by_others_are_protected() {
        let temp_path = tempdir().unwrap().into_path();
        let file_path = temp_path.join("gpg-agent.conf");
        File::create(&file_path).unwrap();

        let command_output = Command::new("chmod")
            .arg("-R")
            .arg("ag+wrx")
            .arg(&temp_path)
            .status()
            .expect("Changing permissions failed?");

        assert_eq!(command_output.code().unwrap(), 0);
        assert!(fix_gnupg_permissions(|| temp_path.to_str().unwrap().to_string()).is_ok());

        let actual = fs::metadata(temp_path).unwrap().permissions().mode();

        assert_eq!(
            actual & PERMISSIONS_ONLY_MASK,
            USER_PLUS_LIST_OCTLET,
            "Expected {:o} found {:o}",
            USER_PLUS_LIST_OCTLET,
            actual & PERMISSIONS_ONLY_MASK
        );
    }
}
