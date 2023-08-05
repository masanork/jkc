use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read, Write};
use std::process;

const VERSION: &'static str = "1.0.0";
const MAPPING: &'static str = include_str!("juki2mj.csv");

fn main() -> io::Result<()> {
    let mut input: Box<dyn Read> = Box::new(io::stdin());
    let mut output: Box<dyn Write> = Box::new(io::stdout());

    let args: Vec<String> = std::env::args().collect();
    let mut i = 1;

    while i < args.len() {
        match args[i].as_str() {
            "-h" | "--help" => {
                print_help();
                process::exit(0);
            }
            "-v" | "--version" => {
                println!("住基ネット統一文字コンバーター バージョン {}", VERSION);
                process::exit(0);
            }
            "-i" | "--input" => {
                i += 1;
                let file_name = args.get(i).expect("入力ファイル名が必要です");
                input = Box::new(File::open(file_name)?);
            }
            "-o" | "--output" => {
                i += 1;
                let file_name = args.get(i).expect("出力ファイル名が必要です");
                output = Box::new(File::create(file_name)?);
            }
            _ => {}
        }
        i += 1;
    }

    let mapping = create_mapping();
    let mut text = String::new();
    input.read_to_string(&mut text)?;

    let converted_text = text.chars().map(|c| mapping.get(&c).unwrap_or(&c).clone()).collect::<String>();

    write!(output, "{}", converted_text)?;

    Ok(())
}

fn create_mapping() -> HashMap<char, char> {
    let mut mapping = HashMap::new();
    for line in MAPPING.lines() {
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() == 2 {
            let from = parts[0].chars().next().unwrap();
            let to = parts[1].chars().next().unwrap();
            mapping.insert(from, to);
        }
    }
    mapping
}

fn print_help() {
    println!("住基ネット統一文字コンバーターの使用方法:");
    println!("  -i, --input <FILE>    入力ファイル名を指定します");
    println!("  -o, --output <FILE>   出力ファイル名を指定します");
    println!("  -v, --version         バージョン情報を表示します");
    println!("  -h, --help            このヘルプメッセージを表示します");
}
