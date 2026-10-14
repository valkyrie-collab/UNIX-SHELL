use std::io::{self, Write};
use std::env;
use std::process::{self, Command, ExitStatus};
use std::path::Path;
use std::str::SplitWhitespace;

fn handle_folder_navigation(args: &[&str]) {
    let cur_folder: String = env::var("HOME").unwrap_or(String::from("/"));
    let folder_to_go: &str = args.first().copied().unwrap_or(&cur_folder);
    let path_of_folder: &Path = Path::new(folder_to_go);

    if let Err(xer) = env::set_current_dir(path_of_folder) {
        eprintln!("cd | {} : {}", folder_to_go, xer);
    }

}

fn handle_exit(args: &[&str]) {
    let code: i32 = args.first().copied().and_then(|xfs| xfs.parse().ok()).unwrap_or(0);

    process::exit(code);
}

fn handle_other_command(cmd: &str, args: &[&str]) {
    let res_cmd: Result<ExitStatus, std::io::Error> = Command::new(cmd).args(args).status();

    match res_cmd {
        Ok(xrc) => {
            
            if !xrc.success() {

                if let Some(xcd) = xrc.code() {
                    eprintln!("Process exited with code {}", xcd);
                }

            }

        }
        Err(xer) => {
            eprintln!("{} | {}", cmd, xer);
        }
    }

}

fn main() {
    println!("My Custom Shell :)");
    
    loop {
        let mut buf: String = String::new();

        print!("myshell> ");
        io::stdout().flush().expect("Failed to flush newline");
        io::stdin().read_line(&mut buf).expect("Failed to read input");

        let buf_trim: &str = buf.trim();
        if buf_trim.is_empty() { continue; }
        let mut part: SplitWhitespace = buf_trim.split_whitespace();
        let cmd: &str = if let Some(xcd) = part.next() { xcd } else { continue; };
        let args: Vec<&str> = part.collect();

        match cmd {
            "cd" => handle_folder_navigation(&args),
            "exit" => handle_exit(&args),
            _ => handle_other_command(cmd, &args),
        }
        
    }
}