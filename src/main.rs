use std::{
    io::{self, Read, Write},
    process::{Command, Stdio},
    os::windows::process::CommandExt,
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
    let mut full_args = vec![];
    full_args.extend(args.iter().map(|s| s.as_str()));
    
    // 检查是否包含 --file - 参数
    let has_file_arg = args.windows(2).any(|w| w[0] == "--file" && w[1] == "-");
    
    // 准备子进程
    let mut child = Command::new("git")
        .args(&full_args)
        .stdin(if has_file_arg { Stdio::piped() } else { Stdio::inherit() })
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .creation_flags(0x08000000) // 禁用Windows控制台缓冲区
        .spawn()?;

    // 如果有 --file - 参数，传递标准输入
    if has_file_arg {
        // 使用缓冲读写器提高效率
        let mut stdin = io::stdin().lock();
        let mut child_stdin = child.stdin.take().unwrap();
        
        // 使用缓冲复制
        let mut buffer = [0; 4096];
        loop {
            let bytes_read = stdin.read(&mut buffer)?;
            if bytes_read == 0 {
                break;
            }
            child_stdin.write_all(&buffer[..bytes_read])?;
        }
        
        // 确保所有数据都写入
        child_stdin.flush()?;
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
