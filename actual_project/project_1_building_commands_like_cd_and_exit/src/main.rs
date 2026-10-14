use std::env;
use std::io::{self, Write};
use std::path::Path;
use std::process::{self, Command};
use std::str::SplitWhitespace;

fn handle_cd(args: &[&str]) {
    let home_dir: String = env::var("HOME").unwrap_or_else(
        |_xer| String::from("/")
    );

    let target_str: &str = args.first().copied().unwrap_or(&home_dir);
    let target_path: &Path = Path::new(target_str);

    if let Err(xer) = env::set_current_dir(target_path) {
        eprintln!("cd: {}: {}", target_str, xer);
    }
}

fn handle_exit(args: &[&str]) {
    let code: i32 = args.first().copied().and_then(|arg| arg.parse().ok()).unwrap_or(0);
    process::exit(code);
}

fn execute_external(command: &str, args: &[&str]) {

    match Command::new(command).args(args).status() {
        Ok(xcmd) => {

            if !xcmd.success() {

                if let Some(xcode) = xcmd.code() {
                    eprintln!("Process exited with status code {}", xcode);
                }

            }

        }
        Err(xer) => {
            eprintln!("{}: {}", command, xer);
        }
    }

}

fn main() {
    loop {
        print!("myshel> ");
        io::stdout().flush().expect("failed to flush stdout");

        let mut input: String = String::new();

        if io::stdin().read_line(&mut input).expect("Failed to read line") == 0 {
            //EOF reached;
            println!();
            break;
        }

        let trimmed: &str = input.trim();

        if trimmed.is_empty() {
            continue;
        }

        let mut parts: SplitWhitespace = trimmed.split_whitespace();
        let command: &str = if let Some(cmd) = parts.next() { cmd } else { continue; };
        let args: Vec<&str> = parts.collect();

        match command {
            "cd" => handle_cd(&args),
            "exit" => handle_exit(&args),
            _ => execute_external(command, &args)
        }
    }
}
