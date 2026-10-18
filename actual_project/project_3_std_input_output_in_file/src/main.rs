use std::fs;
use std::io::Write;
use std::path::Path;
use std::process;
use std::io;
use std::env;
use std::process::Command;
use std::process::Stdio;
use std::str::SplitWhitespace;

fn tokenize(input: &str) -> io::Result<Vec<String>> {
    let mut is_escaped: bool = false;
    let mut is_single_quotes: bool = false;
    let mut is_double_quotes: bool = false;
    let mut curr_token: String = String::new();
    let mut tokens: Vec<String> = Vec::new();
    let mut iter_input: std::iter::Peekable<std::str::Chars> = input.chars().peekable();

    while let Some(xch) = iter_input.next() {
        
        if is_escaped {
            curr_token.push(xch);
            is_escaped = false;
            continue;
        }

        if is_single_quotes {

            if xch == '\'' {
                is_single_quotes = false;
            } else {
                curr_token.push(xch);
            }

            continue;
        }

        if is_double_quotes {

            match xch {
                '\\' => is_escaped = true,
                '"' => is_double_quotes = false,
                _ => curr_token.push(xch),
            }

            continue;
        }

        match xch {
            '\\' => is_escaped = true,
            '"' => is_double_quotes = true,
            '\'' => is_single_quotes = true,
            ' ' => {

                if !curr_token.is_empty() {
                    tokens.push(curr_token.clone());
                    curr_token.clear();
                }

            }
            _ => curr_token.push(xch),
        }

    }

    if !curr_token.is_empty() {
        tokens.push(curr_token);
    }

    if is_single_quotes {
        return Err(io::Error::other("Syntax error! cannot end with single quotes"));
    }    

    if is_double_quotes {
        return Err(io::Error::other("Syntax error! cannot end with double quotes"));
    }

    Ok(tokens)
}

fn handle_cd(args: &[String]) {
    let cur_dir: String = env::var("HOME").unwrap_or(String::from("/"));
    let target_dir: &str = args.first().unwrap_or(&cur_dir);
    let target_path: &Path = Path::new(target_dir);

    if let Err(xer) = env::set_current_dir(target_path) {
        eprintln!("Error! cd: {} | {}", target_dir, xer);
    }

}

fn handle_exit(args: &[String]) {
    let exit_code: i32 = args.first().and_then(|xec| xec.parse().ok()).unwrap_or(0);

    process::exit(exit_code);
}

fn handle_external_cmd(cmd: &str, args: &[String]) {
    let mut init_cmd: process::Command = process::Command::new(cmd);
    let cmd: &mut process::Command = init_cmd.args(args);

    match cmd.status() {
        Ok(xes) => {

            if !xes.success() {
                eprintln!("Error cmd did not ran successfully");
            }

        }

        Err(xer) => {
            eprintln!("Error! exited with error {}", xer);
        }
    }
}

fn handle_external_cmd_write_in_file(cmd: &str, args: &[String], file_path: &str) {
    let file: fs::File = match fs::OpenOptions::new()
    .create(true).write(true).append(true).read(true).open(file_path) {
        Ok(xfo) => xfo,
        Err(xer) => {
            eprintln!("Failed to create file, Exited with error {}", xer);
            return;
        }
    };

    let mut init_cmd: process::Command = process::Command::new(cmd);
    let cmd: &mut process::Command = init_cmd.args(args);

    match cmd.stdout(process::Stdio::from(file)).status() {
        Ok(xes) => {

            if !xes.success() {
                eprintln!("Error cmd did not ran successfully");
            }

        }

        Err(xer) => {
            eprintln!("Error! exited with error {}", xer);
        }
    }

}

fn handle_external_cmd_write_from_file(cmd: &str, args: &[String], file_path: &str) {
    let file: fs::File = match fs::OpenOptions::new().read(true).open(file_path) {
        Ok(xfs) => xfs,
        Err(xer) => {
            eprintln!("Failed to create file, Exited with error {}", xer);
            return;
        }
    };

    let mut init_cmd: Command = Command::new(cmd);
    let cmd: &mut Command = init_cmd.args(args);

    match cmd.stdin(Stdio::from(file)).status() {
        Ok(xes) => {

            if !xes.success() {
                eprintln!("Failed to run command");
            }
        }
        Err(xer) => {
            eprintln!("Failed to run command exited with error {}", xer);
        }
    }

}

fn main() {
    println!("My custom UNIX SHELL");

    loop {
        let mut buf: String = String::new();

        print!("myshell> ");
        io::stdout().flush().expect("cannot clear newline");
        io::stdin().read_line(&mut buf).expect("cannot write the buf");

        let mut trim_part: SplitWhitespace = buf.trim().split_whitespace();
        let cmd: &str = if let Some(xtr) = trim_part.next() { xtr } else { continue; };
        let temp_args: String = trim_part.collect();

        if temp_args.contains(">") {
            let (args, file_path): (&str, &str) = if let Some(xtr) = temp_args.split_once('>') {
                xtr 
            } else { continue; };

            let args: Vec<String> = match tokenize(args) {
                Ok(xtn) => xtn,
                Err(xer) => {
                    eprintln!("Error! exited with {}", xer);
                    continue;
                }
            };

            handle_external_cmd_write_in_file(cmd, &args, file_path);
        } else if temp_args.contains("<") {
            let (args, file_path): (&str, &str) = if let Some(xtr) = temp_args.split_once('<') {
                xtr 
            } else { continue; };

            let args: Vec<String> = match tokenize(args) {
                Ok(xtn) => xtn,
                Err(xer) => {
                    eprintln!("Error! exited with {}", xer);
                    continue;
                }
            };

            handle_external_cmd_write_from_file(cmd, &args, file_path);
        } else {
            let args: Vec<String> = match tokenize(&temp_args) {
                Ok(xtn) => xtn,
                Err(xer) => {
                    eprintln!("Error! exited with {}", xer);
                    continue;
                }
            };

            match cmd {
                "cd" => handle_cd(&args),
                "exit" => handle_exit(&args),
                _ => handle_external_cmd(cmd, &args),
            }
        }

    }
}
