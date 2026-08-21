fn main() {
    #[cfg(not(debug_assertions))]
    {
        // Ensure that the build script is only run when not in debug mode
        // as tests and debug builds may also be run on non-supported platforms
        // as long as specific features are not used
        ensure_target_is_supported();
    }

    if let Some((date, short_id)) = get_commit_information() {
        println!("cargo:rustc-env=BUILD_COMMIT_DATE={}", date);
        println!("cargo:rustc-env=BUILD_COMMIT_ID={}", short_id);
    } else {
        eprintln!("cargo:warning=Failed to retrieve git commit information");
    }
}

fn ensure_target_is_supported() {
    let target_os =
        std::env::var("CARGO_CFG_TARGET_OS").expect("Expected CARGO_CFG_TARGET_OS to be set");

    if target_os != "linux" {
        panic!("x-usb-bridge only supports linux at the moment (last updated: 08/2026)");
    }
}

fn get_commit_information() -> Option<(String, String)> {
    let output = std::process::Command::new("git")
        .args(["log", "-1", "--format=%cd|%h", "--date=short"])
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    let stdout = String::from_utf8(output.stdout).ok()?;
    let parts: Vec<&str> = stdout.trim().split('|').collect();

    if parts.len() != 2 {
        return None;
    }

    Some((
        parts[0].trim().to_string(), // date
        parts[1].trim().to_string(), // short id
    ))
}
