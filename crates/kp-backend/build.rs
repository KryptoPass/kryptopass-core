use std::{env, path::PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-env-changed=KP_SYNTHETIC_FIXTURE_DIR");
    println!("cargo:rerun-if-env-changed=XDG_CACHE_HOME");
    println!("cargo:rerun-if-env-changed=HOME");

    let manifest_directory = PathBuf::from(
        env::var_os("CARGO_MANIFEST_DIR").ok_or("Cargo manifest directory is missing")?,
    );
    let fixture_directory = synthetic_fixture_directory()?;
    let fixture_directory = if fixture_directory.is_absolute() {
        fixture_directory
    } else {
        manifest_directory.join(fixture_directory)
    };
    let fixture_manifest = fixture_directory.join("fixture.json");
    let fixture_directory_text = fixture_directory
        .to_str()
        .ok_or("synthetic fixture path is not valid UTF-8")?;
    let fixture_manifest_text = fixture_manifest
        .to_str()
        .ok_or("synthetic fixture manifest path is not valid UTF-8")?;
    println!("cargo:rustc-env=KP_SYNTHETIC_FIXTURE_DIR={fixture_directory_text}");
    println!("cargo:rustc-env=KP_SYNTHETIC_FIXTURE_MANIFEST={fixture_manifest_text}");
    Ok(())
}

/// The same location kp-desktop's build.rs resolves, so both see one fixture.
fn synthetic_fixture_directory() -> Result<PathBuf, Box<dyn std::error::Error>> {
    if let Some(directory) = env::var_os("KP_SYNTHETIC_FIXTURE_DIR") {
        return Ok(PathBuf::from(directory));
    }
    let cache_home = env::var_os("XDG_CACHE_HOME")
        .map(PathBuf::from)
        .or_else(|| env::var_os("HOME").map(|home| PathBuf::from(home).join(".cache")))
        .ok_or("HOME is unavailable for the synthetic fixture cache")?;
    Ok(cache_home.join("kryptopass/synthetic-sync-v1"))
}
