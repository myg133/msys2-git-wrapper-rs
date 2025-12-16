use std::{
    io::{self, Read, Write},
    process::{Command, Stdio},
};

fn main() -> io::Result<()> {
    let args: Vec<String> = std::env::args().skip(1).collect();

    if args.contains(&"rev-parse".to_string()) || args.contains(&"ls-files".to_string()) {
        handle_path_conversion(&args)
    } else if args.get(0) == Some(&"commit".to_string()) {
        handle_commit_command(&args)
    } else {
        handle_generic_command(&args)
    }
}

fn handle_commit_command(args: &[String]) -> io::Result<()> {
    // 确保包含必要的配置参数
    let mut full_args = vec!["-c", "user.useConfigOnly=true"];
    full_args.extend(args.iter().map(|s| s.as_str()));
    
    // 检查是否包含 --file - 参数
    let has_file_arg = args.windows(2).any(|w| w[0] == "--file" && w[1] == "-");
    
    // 准备子进程
    let mut child = Command::new("git")
        .args(&full_args)
        .stdin(if has_file_arg { Stdio::piped() } else { Stdio::inherit() })
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()?;

    // 如果有 --file - 参数，传递标准输入
    if has_file_arg {
        let mut message = String::new();
        io::stdin().read_to_string(&mut message)?;
        
        // 验证消息非空（除非允许空消息）
        let allow_empty = args.contains(&"--allow-empty-message".to_string());
        if message.trim().is_empty() && !allow_empty {
            eprintln!("错误：提交消息不能为空");
            std::process::exit(1);
        }
        
        let mut stdin = child.stdin.take().unwrap();
        stdin.write_all(message.as_bytes())?;
    }

    let status = child.wait()?;
    if !status.success() {
        std::process::exit(status.code().unwrap_or(1));
    }
    Ok(())
}

fn handle_path_conversion(args: &[String]) -> io::Result<()> {
    let git_output = Command::new("git")
        .args(args)
        .output()?;
    
    if git_output.status.success() {
        let unix_path = String::from_utf8_lossy(&git_output.stdout);
        let windows_path_output = Command::new("cygpath")
            .args(&["-w", unix_path.trim()])
            .output()?;
        
        let windows_path = String::from_utf8_lossy(&windows_path_output.stdout)
            .trim()
            .to_string();
        println!("{}", windows_path);
        Ok(())
    } else {
        io::stderr().write_all(&git_output.stderr)?;
        std::process::exit(git_output.status.code().unwrap_or(1));
    }
}

fn handle_generic_command(args: &[String]) -> io::Result<()> {
    let output = Command::new("git")
        .args(args)
        .output()?;
    
    if output.status.success() {
        io::stdout().write_all(&output.stdout)?;
    } else {
        io::stderr().write_all(&output.stderr)?;
    }
    
    if !output.status.success() {
        std::process::exit(output.status.code().unwrap_or(1));
    }
    Ok(())
}
