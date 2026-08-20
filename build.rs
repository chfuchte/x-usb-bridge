fn main() {
    if let Some((date, short_id)) = commit_information() {
        println!("cargo:rustc-env=BUILD_COMMIT_DATE={}", date);
        println!("cargo:rustc-env=BUILD_COMMIT_ID={}", short_id);
    } else {
        eprintln!("cargo:warning=Failed to retrieve git commit information");
    }
}

fn commit_information() -> Option<(String, String)> {
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
