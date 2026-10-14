use std::io::{self, Write, Error};

fn tokenize(input: &str) -> Result<Vec<String>, std::io::Error> {
    let mut token: Vec<String> = Vec::new();
    let mut current_token: String = String::new();
    let mut in_single_quotes: bool = false;
    let mut in_double_quotes: bool = false;
    let mut is_escaped: bool = false;
    let mut chars: std::iter::Peekable<std::str::Chars<'_>> = input.chars().peekable();

    while let Some(ch) = chars.next() {

        if is_escaped {
            current_token.push(ch);
            is_escaped = false;
            continue;
        }

        if in_single_quotes {

            if ch == '\'' {
                in_single_quotes = false;
            } else {
                current_token.push(ch);
            }

            continue;
        }

        if in_double_quotes {
            
            match ch {
                '\\' => is_escaped = true,
                '"' => in_double_quotes = false,
                _ => current_token.push(ch),
            }

            continue;
        }

        match ch {
            '\\' => is_escaped = true,
            '\'' => in_single_quotes = true,
            '"' => in_double_quotes = true,
            ' ' => {

                if !current_token.is_empty() {
                    token.push(current_token.clone());
                    current_token.clear();
                }

            }
            _ => current_token.push(ch),
        }

    }


    if !current_token.is_empty() {
        token.push(current_token);
    }

    if in_single_quotes {
        let err: Error = Error::other("Syntax Error: Unclosed single quote");
        return Err(err);
    }

    if in_double_quotes {
        let err: Error = Error::other("Syntax Error: Unclosed double quote");
        return Err(err);
    }

    Ok(token)
}

fn main() {
    println!("My custom shell");

    loop {
        let mut buf: String = String::new();

        print!("myshell> ");
        io::stdout().flush().expect("cannot flush newline");
        io::stdin().read_line(&mut buf).expect("cannot read from terminal");

        let res: Result<Vec<String>, io::Error> = tokenize(&buf);

        println!("{:?}", res.unwrap_or_else(|xer| vec![xer.to_string()]))
    }

}
