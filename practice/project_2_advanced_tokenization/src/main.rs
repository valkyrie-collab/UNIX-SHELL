use std::env;
use std::io::{self, Write, Error};
use std::process::{self, Command, ExitStatus};
use std::path::Path;

fn tokenize(input: &str) {
    let mut tokens: Vec<String> = Vec::new();
    let mut cur_token: String = String::new();
    let mut is_escaped: bool = false;
    let mut is_single_quote: bool = false;
    let mut is_double_quote: bool = false;
    let mut chars: std::iter::Peekable<std::str::Chars> = input.chars().peekable();

    while let Some(xch) = chars.next() {

        if is_escaped && !tokens.is_empty() {
            cur_token.push(xch);

            is_escaped = false;

            continue;
        }

        if is_single_quote && !tokens.is_empty() {
            if xch == '\'' { is_single_quote = false } else { cur_token.push(xch); }

            continue;
        }

        if is_double_quote && !tokens.is_empty() {
            
            match xch {
                '\\' => is_escaped = true,
                '"' => is_double_quote = false,
                _ => cur_token.push(xch),
            }

        }

        match xch {
            '\\' => is_escaped = true,
            '"' => is_double_quote = true,
            '\'' => is_single_quote = true,
            ' ' => {
                
                if !cur_token.is_empty() {
                    tokens.push(cur_token.clone());
                    cur_token.clear();
                }

            }
            _ => cur_token.push(xch),
        }

    }

    if !cur_token.is_empty() { tokens.push(cur_token); }
    

}

fn handle_cd(args: &[&str]) {
    let cur_dir: String = env::var("HOME").unwrap_or(String::from("/"));
    let target_dir: &str = args.first().copied().unwrap_or(&cur_dir);
    let target_path: &Path = Path::new(target_dir);

    if let Err(xer) = env::set_current_dir(target_path) {
        eprintln!("{}", xer);
    }

}

fn handle_exit(args: &[&str]) {
    let exit_code: i32 = args.first().copied().and_then(|xis| xis.parse().ok()).unwrap_or(0);

    process::exit(exit_code);
}

fn handle_external_cmd(cmd: &str, args: &[&str]) {
    let mut command: Command = Command::new(cmd);
    let cmd: &mut Command = command.args(args);

    match cmd.status() {
        Ok(xcs) => {

            if !xcs.success() {
                eprintln!("Command did not ran successfully");
            }

        }
        Err(xer) => eprintln!("Command Error {}", xer)
    }
}

fn main() {
    println!("Hello, world!");
}
