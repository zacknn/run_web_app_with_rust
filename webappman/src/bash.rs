use std::process::Command;

pub fn launch_app(url: &str) {
    Command::new("bash")
        .arg("scripts/lunch.sh")
        .arg(url)
        .spawn()
        .expect("failed to launch");
}

pub fn add_app(name: &str, url: &str) {
    Command::new("bash")
        .arg("scripts/add.sh")
        .arg(name)
        .arg(url)
        .output()
        .expect("failed to add");
}

pub fn remove_app(name: &str) {
    Command::new("bash")
        .arg("scripts/remove.sh")
        .arg(name)
        .output()
        .expect("failed to remove");
}
