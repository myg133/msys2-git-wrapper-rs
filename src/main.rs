use std::process::Command;

fn main() {
    let mut args: Vec<String> = std::env::args().collect();
    args.remove(0);

    if args.contains(&String::from("rev-parse")) || args.contains(&String::from("ls-files")) {
        let git_output = Command::new("git")
           .args(&args)
           .output()
           .expect("Failed to execute git rev-parse");
        if git_output.status.success() {
            let unix_path = String::from_utf8_lossy(&git_output.stdout);
            let windows_path_output = Command::new("cygpath")
               .args(&["-w", unix_path.trim()])
               .output()
               .expect("Failed to convert path using cygpath");
            let windows_path = String::from_utf8_lossy(&windows_path_output.stdout).trim().to_string();
            println!("{}", windows_path);
        } else {
            eprintln!("Command failed with error: {}", git_output.status);
        }
    } else {
        let output = Command::new("git")
           .args(&args)
           .output()
           .expect("Failed to execute git command");
        println!("{}", String::from_utf8_lossy(&output.stdout));
        if output.status.success() {
            println!("{}", String::from_utf8_lossy(&output.stdout));
        } else {
            eprintln!("Command failed with error: {}", output.status);
        }
    }
}