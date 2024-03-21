use regex::{Error, Regex};
use std::{env, io};

fn main() {
    let env: Vec<_> = env::args().skip(1).collect();
    if env.is_empty() {
        eprintln!("チェックする正規表現を引数に指定してください");
        return;
    }
    let regex = Regex::new(env[0].as_str());
    match regex {
        Ok(reg) => {
            dialogue(reg);
        }
        Err(e) => match e {
            Error::Syntax(_) => {
                eprintln!("間違った正規表現が入力されました。正しい正規表現を使用してください。")
            }
            Error::CompiledTooBig(_) => eprintln!("この正規表現は大きすぎます。"),
            _ => {}
        },
    }
}

fn dialogue(regex: Regex) {
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input.pop();

        let mut fmt = String::new();
        regex
            .find_iter(input.as_str())
            .map(|m| m.as_str())
            .for_each(|s| {
                fmt = format!("{}\"{}\", ", fmt, s);
            });
        fmt.pop();
        if fmt.is_empty() {
            println!(">> No matches");
        } else {
            println!(">> Match {}", fmt);
        }
    }
}
