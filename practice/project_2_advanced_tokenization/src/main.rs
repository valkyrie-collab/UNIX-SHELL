use std::env;
use std::io::{self, Write, Error};
use std::path::Path;
use std::process::{self, ExitCode, Command};
use std::str::SplitWhitespace;

fn tokenize(input: &str) -> std::io::Result<Vec<String>> {
    let mut tokens: Vec<String> = Vec::new();
    let mut current_token: String = String::new();
    let mut is_escaped: bool = false;
    let mut is_single_quote: bool = false;
    let mut is_double_quote: bool = false;
    let mut iter_input: std::iter::Peekable<std::str::Chars<'_>> = input.chars().peekable();

    while let Some(xch) = iter_input.next() {
        
        if is_escaped {
            current_token.push(xch);
            is_escaped = false;
            continue;
        }

        if is_single_quote {
            if xch == '\'' { is_single_quote = false } else { current_token.push(xch); }
            continue;
        }

        if is_double_quote {

            match xch {
                '\\' => is_escaped = true,
                '"' => is_double_quote = false,
                _ => current_token.push(xch),
            }

            continue;
        }

        match xch {
            '\'' => is_single_quote = true,
            '"' => is_double_quote = true,
            '\\'=> is_escaped = true,
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
        let err: Error = Error::other("Syntax Error! cannot end with open single quotes");
        return Err(err);
    }
    
    if is_double_quote {
        let err: Error = Error::other("Syntax Error! cannot end open with double quotes");
        return Err(err);
    }

    Ok(tokens)
}

fn handle_cd(args: &[String]) {
    let curr_dir: String = env::var("HOME").unwrap_or(String::from("/"));
    let target_dir: &str = args.first().unwrap_or(&curr_dir);
    let target_path: &Path = Path::new(target_dir);

    if let Err(xer) = env::set_current_dir(target_path) {
        eprintln!("Error! cd: {} | {}", target_dir, xer);
    }

}

fn handle_exit(args: &[String]) {
    let exit_code: i32 = args.first().and_then(|xec| xec.parse().ok()).unwrap_or(0);

    process::exit(exit_code);
}

fn handle_other_cmd(cmd: &str, args: &[String]) {
    let mut cmd: Command = Command::new(cmd);
    let cmd: &mut Command = cmd.args(args);

    match cmd.status() {
        Ok(xcs) => {
            
            if !xcs.success() {
                eprintln!("Error! cmd did not work check again");
            }

        }
        Err(xer) => {
            eprintln!("Exited with error {}", xer);
        }
    }

}

fn main() {
    println!("My custom unix shell");

    loop {
        let mut buf: String = String::new();

        print!("myshell> ");
        io::stdout().flush().expect("cannot remove new line");
        io::stdin().read_line(&mut buf).expect("error while writing buffer");

        let mut part: SplitWhitespace = buf.split_whitespace();
        let cmd: &str = if let Some(xpn) = part.next() { xpn } else { continue; };
        let args: String = part.collect();
        let tkn_args: Vec<String> = tokenize(args.trim()).unwrap_or(Vec::new());
        
        match cmd {
            "cd" => handle_cd(&tkn_args),
            "exit" => handle_exit(&tkn_args),
            _ => handle_other_cmd(cmd, &tkn_args),
        }

    }
}
