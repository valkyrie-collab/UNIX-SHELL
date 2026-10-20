use std::io::{self, Write};
use std::fs;
use std::path::Path;
use std::process;
use std::env;

fn tokenize(input: &str) -> Result<Vec<String>, io::Error>{
    let mut is_escaped: bool = false;  
    let mut is_single_quote: bool = false;
    let mut is_double_quotes: bool = false;
    let mut current_token: String = String::new();
    let mut tokens: Vec<String> = Vec::new();
    let mut init_token: std::iter::Peekable<std::str::Chars> = input.chars().peekable();

    while let Some(xch) = init_token.next() {

        if is_escaped {
            is_escaped = false;
            current_token.push(xch);
            continue;
        }

        if is_single_quote {

            if xch == '\'' {
                is_single_quote = false;
            } else {
                current_token.push(xch);
            }

            continue;
        }

        if is_double_quotes {

            match xch {
                '\\' => is_escaped = true,
                '"' => is_double_quotes = false,
                _ => current_token.push(xch),
            }

            continue;
        }

        match xch {
            '\\' => is_escaped = true,
            '"' => is_double_quotes = true,
            '\'' => is_single_quote = true,
            '>' => {
                current_token.push(xch);
                tokens.push(current_token.clone());
                current_token.clear();
            }
            '<' => {
                current_token.push(xch);
                tokens.push(current_token.clone());
                current_token.clear();
            }
            ' ' => {

                if !current_token.is_empty() {
                    tokens.push(current_token.clone());
                    current_token.clear();
                }

            }
            _ => current_token.push(xch),
        }

    }

    if !current_token.is_empty() {
        tokens.push(current_token);
    }

    if is_single_quote {
        return Err(io::Error::other("Syntax Error! cannot end with single quotes start"));
    }

    if is_double_quotes {
        return Err(io::Error::other("Syntax Error! cannot end with double quotes start"));
    }

    Ok(tokens)
}

fn handle_cd(args: &[String], cur_dir: &mut String) {
    let curr_dir: String = env::var("HOME").unwrap_or(String::from("/"));
    let target_dir: &str = args.first().unwrap_or(&curr_dir);
    let target_path: &Path = Path::new(target_dir);

    if let Err(xer) =  env::set_current_dir(target_path) {
        eprintln!("Directory movement did not work, Exited with error {}", xer);
        return;
    }

    cur_dir.clear();
    cur_dir.push_str(target_dir);
}

fn handle_exit(args: &[String]) {
    let exit_code: i32 = args.first().and_then(|xvl| xvl.parse().ok()).unwrap_or(0);

    process::exit(exit_code);
}

fn handle_write_in_file(cmd: &str, args: &[String], file_path: &str) {
    let file: fs::File = match fs::OpenOptions::new().create(true).write(true).open(file_path) {
        Ok(xfs) => xfs,
        Err(xer) => {
            eprintln!("File Error! {}", xer);
            return;
        }
    };

    let mut init_cmd: process::Command = process::Command::new(cmd);
    let new_cmd: &mut process::Command = init_cmd.args(args);

    match new_cmd.stdout(process::Stdio::from(file)).status() {
        Ok(xes) => {

            if !xes.success() {
                eprintln!("Command accepted but did not run");
            }

        }
        Err(xer) => {
            eprintln!("CMD: {} did not ran exited with error {}", cmd, xer);
        }
    }

}

fn handle_read_from_file(cmd: &str, args: &[String], file_path: &str) {
    let file: fs::File = match fs::OpenOptions::new().read(true).open(file_path) {
        Ok(xfs) => xfs,
        Err(xer) => {
            eprintln!("File Error! {}", xer);
            return;
        }
    };

    let mut init_cmd: process::Command = process::Command::new(cmd);
    let new_cmd: &mut process::Command = init_cmd.args(args);

    match new_cmd.stdin(process::Stdio::from(file)).status() {
        Ok(xes) => {

            if !xes.success() {
                eprintln!("Command accepted but did not run");
            }

        }
        Err(xer) => {
            eprintln!("CMD: {} did not ran exited with error {}", cmd, xer);
        }
    }
    
}

fn parser_checker(cmd: &str, args: &[String]) -> bool {

    if let Some(xps) = args.iter().position(|xst| xst == ">") {
        let new_args: &[String] = &args[0..xps];
        let file_path: &str = &args[xps + 1];

        handle_write_in_file(cmd, new_args, file_path);
        return true;
    }

    if let Some(xps) = args.iter().position(|xst| xst == "<") {
        let new_args: &[String] = &args[0..xps];
        let file_path: &str = &args[xps + 1];

        handle_read_from_file(cmd, new_args, file_path);
        return true;
    }

    false
}

fn handle_external_cmd(cmd: &str, args: &[String]) {
    let mut init_cmd: process::Command = process::Command::new(cmd);
    let new_cmd: &mut process::Command = init_cmd.args(args);

    match new_cmd.status() {
        Ok(xes) => {

            if !xes.success() {
                eprintln!("Error! command accepted but did not run");
            }

        }
        Err(xer) => {
            eprintln!("Error! command not accepted; Exited with error {}", xer);
        }
    }

}

fn main() {
    println!("My custom shell");
    let mut cur_dir: String = env::var("HOME").unwrap_or(String::from("/"));
    env::set_current_dir(Path::new(&cur_dir)).unwrap();

    loop {
        let mut buf: String = String::new();

        print!("myshell:{}> ", &cur_dir);
        io::stdout().flush().expect("cannot clean newline");
        io::stdin().read_line(&mut buf).expect("cannot read user input");

        let (cmd, init_args): (&str, &str) = if let Some(xss) = buf.trim().split_once(' ') {
            xss
        } else { continue; };

        let args: Vec<String> = match tokenize(init_args) {
            Ok(xvs) => xvs,
            Err(xer) => {
                eprintln!("Error, tokenization {}", xer);
                continue;
            }
        };

        if !parser_checker(cmd, &args) {
            
            match cmd {
                "cd" => handle_cd(&args, &mut cur_dir),
                "exit" => handle_exit(&args),
                _ => handle_external_cmd(cmd, &args),
            }
            
        }
    }

}
