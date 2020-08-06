use std::os::unix::fs::PermissionsExt;
use std::{fs, io::Result, path::Path};

fn main() -> Result<()> {
    // Tell Cargo that if the given file changes, to rerun this build script.
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=bsdfan.conf");
    println!("cargo:rerun-if-changed=rc.d/bsdfan");

    let etc = "/usr/local/etc/";
    //let sbin = "/usr/local/sbin/";
    let startup = "rc.d/bsdfan";
    let config = "bsdfan.conf";

    fs::copy(startup, format!("{}{}", etc, startup))?;

    let start_file = fs::File::open(format!("{}{}", etc, startup))?;
    let mut perm = start_file.metadata()?.permissions();
    perm.set_mode(0o755);
    start_file.set_permissions(perm)?;

    let etc_conf = format!("{}{}", etc, config);
    if !Path::new(&etc_conf).is_file() {
        fs::copy(config, etc_conf)?;
    }

    Ok(())
}
