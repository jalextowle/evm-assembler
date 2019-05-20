use regex::Regex;
use std::env;
use std::fs::File;
use std::io::{BufReader, BufRead};

fn to_nibble(num: u8) -> String {
    match num {
       val@10...15 => return ((val + 87) as char).to_string(),
       val@0...9 => return ((val + 48) as char).to_string(),
       _ => panic!("Invalid nibble")
    }
}

fn to_hex(mut num: u32) -> String {
    let mut result = String::new();
    while num > 0 {
        result.insert_str(0, &to_nibble((num % 16) as u8));
        num /= 16;
    }
    result
}

// Return a struct representing the next symbol
fn next_symbol(line: &Vec<char>, cur: &mut usize) -> String {
    let mut started = false;
    let mut result = String::new();
    while *cur < line.len() {
        if line[*cur].is_alphanumeric() {
            started = true;
            result.push(line[*cur].to_ascii_lowercase());
        } else if !line[*cur].is_whitespace() {
            panic!("Parse error");    
        } else if started {
            return result;
        }
        *cur += 1;
    }
    result
}

fn capture_group(expression: &Regex, value: &str) -> String {
    let capture = expression.captures_iter(value).nth(0).expect("Unable to capture group");
    capture[1].to_string()
}

fn sized_opcode(expression: &Regex, value: &str, start_size: u32) -> String {
    match capture_group(expression, value).parse::<u32>().unwrap() {
        num@1...16 => to_hex(start_size + num),
        _ => panic!("Invalid opcode size")
    }
}

fn parse(input: BufReader<File>) -> String {
    let mut expect_size = false;
    let mut result = String::from("0x");
    let dup_re = Regex::new(r"dup([0-9]+)").unwrap();
    let push_re = Regex::new(r"push([0-9]+)").unwrap();
    let swap_re = Regex::new(r"swap([0-9]+)").unwrap();
    let hex_re = Regex::new(r"0x([0-9a-f]+)").unwrap();
    for line in input.lines() {
        let mut cur = 0;
        let chars = line.expect("Unable to read line").chars().collect::<Vec<char>>();
        while cur < chars.len() {
            let symbol = next_symbol(&chars, &mut cur);
            if expect_size && hex_re.is_match(symbol.as_ref()) {
                result.push_str(&capture_group(&hex_re, symbol.as_ref()));
                expect_size = false;
            } else {
                match symbol.as_ref() {
                    "stop" => result.push_str("00"),
                    "add" => result.push_str("01"),
                    "mul" => result.push_str("02"),
                    "sub" => result.push_str("03"),
                    "div" => result.push_str("04"),
                    "sdiv" => result.push_str("05"),
                    "mod" => result.push_str("06"),
                    "smod" => result.push_str("07"),
                    "addmod" => result.push_str("08"),
                    "mulmod" => result.push_str("09"),
                    "exp" => result.push_str("0a"),
                    "signextend" => result.push_str("0b"),
                    "lt" => result.push_str("10"),
                    "gt" => result.push_str("11"),
                    "slt" => result.push_str("12"),
                    "sgt" => result.push_str("13"),
                    dup if dup_re.is_match(dup) => result.push_str(&sized_opcode(&dup_re, dup, 0x7f)),
                    push if push_re.is_match(push) => {
                        result.push_str(&sized_opcode(&push_re, push, 0x5f));
                        expect_size = true;
                    }
                    swap if swap_re.is_match(swap) => result.push_str(&sized_opcode(&swap_re, swap, 0x8f)),
                    _ => panic!("Invalid opcode")
                }
            }
        }
    }
    result
}

fn main() {
    let name = env::args().nth(1).expect("Usage: alex FILE_NAME");
    let input = File::open(name).expect("Unable to open input file");
    println!("{}", parse(BufReader::new(input)));
}
