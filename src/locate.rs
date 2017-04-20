use std::process::{Command};
use std::path::{PathBuf};

pub fn get_paths(pattern: &str) -> Vec<PathBuf> {
    if pattern.is_empty() {
        // TODO return Result instead
        Vec::new()
    }
    else {
        let s = call_locate(pattern);
        let paths: Vec<_> = s.lines()
            .map(|x| PathBuf::from(x))
            .collect();
        paths
    }
}


fn call_locate(pattern: &str) -> String {
    let output = Command::new("locate")
        .arg(pattern)
        .output()
        .expect("failed to call `locate`");
    String::from_utf8(output.stdout).expect("not UTF-8")
}
