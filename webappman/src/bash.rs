use std::path::PathBuf;
use std::process::Command;

fn find_script(script: &str) -> PathBuf {
    // Try a few likely locations: ./scripts, ../scripts, search upwards from executable
    let candidates = ["scripts", "../scripts"];

    for c in &candidates {
        let p = PathBuf::from(c).join(script);
        if p.exists() {
            return p;
        }
    }

    // Walk up from current_exe looking for a parent that contains `scripts`
    if let Ok(mut exe) = std::env::current_exe() {
        while exe.pop() {
            let p = exe.join("scripts").join(script);
            if p.exists() {
                return p;
            }
        }
    }

    // Fallback to script name (may fail)
    PathBuf::from("scripts").join(script)
}

pub fn launch_app(url: &str) {
    let script = find_script("launch.sh");
    Command::new("bash")
        .arg(script)
        .arg(url)
        .spawn()
        .expect("failed to launch");
}

pub fn add_app(name: &str, url: &str) {
    let script = find_script("add.sh");
    Command::new("bash")
        .arg(script)
        .arg(name)
        .arg(url)
        .output()
        .expect("failed to add");
}

pub fn remove_app(name: &str) {
    let script = find_script("remove.sh");
    Command::new("bash")
        .arg(script)
        .arg(name)
        .output()
        .expect("failed to remove");
}
