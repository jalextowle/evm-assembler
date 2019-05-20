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

fn sized_opcode(expression: &Regex, value: &str, start_size: u32, max_size: u32) -> String {
    match capture_group(expression, value).parse::<u32>().unwrap() {
        num@1...max_size => to_hex(start_size + num),
        _ => panic!("Invalid opcode size")
    }
}

fn parse(input: BufReader<File>) -> String {
    let mut expect_size = false;
    let mut result = String::from("0x");
    let dup_re = Regex::new(r"dup([0-9]+)").unwrap();
    let log_re = Regex::new(r"log([1-4])").unwrap();
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
                    "eq" => result.push_str("14"),
                    "iszero" => result.push_str("15"),
                    "and" => result.push_str("16"),
                    "or" => result.push_str("17"),
                    "xor" => result.push_str("18"),
                    "not" => result.push_str("19"),
                    "byte" => result.push_str("1a"),
                    "sha3" => result.push_str("20"),
                    "address" => result.push_str("30"),
                    "balance" => result.push_str("31"),
                    "origin" => result.push_str("32"),
                    "caller" => result.push_str("33"),
                    "callvalue" => result.push_str("34"),
                    "calldataload" => result.push_str("35"),
                    "calldatasize" => result.push_str("36"),
                    "calldatacopy" => result.push_str("37"),
                    "codesize" => result.push_str("38"),
                    "codecopy" => result.push_str("39"),
                    "gasprice" => result.push_str("3a"),
                    "extcodesize" => result.push_str("3b"),
                    "extcodecopy" => result.push_str("3c"),
                    "returndatasize" => result.push_str("3d"),
                    "returndatacopy" => result.push_str("3e"),
                    "blockhash" => result.push_str("40"),
                    "coinbase" => result.push_str("41"),
                    "timestamp" => result.push_str("42"),
                    "number" => result.push_str("43"),
                    "difficulty" => result.push_str("44"),
                    "gaslimit" => result.push_str("45"),
                    "pop" => result.push_str("50"),
                    "mload" => result.push_str("51"),
                    "mstore" => result.push_str("52"),
                    "mstore8" => result.push_str("53"),
                    "sload" => result.push_str("54"),
                    "sstore" => result.push_str("55"),
                    "jump" => result.push_str("56"),
                    "jumpi" => result.push_str("57"),
                    "pc" => result.push_str("58"),
                    "msize" => result.push_str("59"),
                    "gas" => result.push_str("5a"),
                    "jumpdest" => result.push_str("5b"),
                    "create" => result.push_str("f0"),
                    "call" => result.push_str("f1"),
                    "callcode" => result.push_str("f2"),
                    "return" => result.push_str("f3"),
                    "delegatecall" => result.push_str("f4"),
                    "staticcall" => result.push_str("fa"),
                    "revert" => result.push_str("fd"),
                    "invalid" => result.push_str("fe"),
                    "selfdestruct" => result.push_str("ff"),
                    dup if dup_re.is_match(dup) => result.push_str(&sized_opcode(&dup_re, dup, 0x7f, 16)),
                    log if log_re.is_match(log) => result.push_str(&sized_opcode(&log_re, log, 0x9f, 4)),
                    push if push_re.is_match(push) => {
                        result.push_str(&sized_opcode(&push_re, push, 0x5f, 32));
                        expect_size = true;
                    }
                    swap if swap_re.is_match(swap) => result.push_str(&sized_opcode(&swap_re, swap, 0x8f, 16)),
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
