use regex::Regex;
use std::collections::*;
use std::iter::*;

pub fn solve() {
    println!("Day 16");

    let lines = adventlib::read_input_lines("day16input.txt");

    let methods: Vec<&Fn(&Vec<i32>, &mut Vec<i32>)> = vec![
        &addr, &addi, &mulr, &muli, &banr, &bani, &borr, &bori, &setr, &seti, &gtri, &gtir, &gtrr,
        &eqri, &eqir, &eqrr,
    ];

    let mut possible_opcodes: Vec<_> = (0..16).map(|_| HashSet::<u8>::from_iter(0..16)).collect();
    let mut actual_opcodes = HashMap::<u8, usize>::new();

    let mut consecutive_blanks = 0;
    let mut program_started = false;
    let mut registers = vec![0; 4];

    let mut count_with_three = 0;
    let mut line_feed = lines.iter().peekable();
    while line_feed.peek() != None {
        let first_line = line_feed.next().unwrap();
        if first_line.trim().is_empty() {
            consecutive_blanks += 1;
            if consecutive_blanks > 1 {
                program_started = true;
            }
            continue;
        } else {
            consecutive_blanks = 0;
        }

        // Part 1 count and opcode mapping
        if first_line.as_str().starts_with("Before") {
            let start_reg = parse_registers(first_line);
            let command = parse_command(line_feed.next().expect("Command line"));
            let end_reg = parse_registers(line_feed.next().expect("Result line"));

            let mut match_count = 0;
            let mut method_index = 0;
            for method in methods.iter() {
                if exec_and_check(method, &command, &start_reg, &end_reg) {
                    match_count += 1;
                } else {
                    possible_opcodes[method_index].remove(&(command[0] as u8));
                }
                method_index += 1;
            }

            if match_count >= 3 {
                count_with_three += 1;
            }
        }

        if program_started && actual_opcodes.len() == 0 {
            while actual_opcodes.len() < 16 {
                for pair in possible_opcodes
                    .iter()
                    .enumerate()
                    .filter(|&(_i, set)| set.len() == 1)
                {
                    actual_opcodes.insert(*pair.1.iter().next().unwrap(), pair.0);
                }
                for set in possible_opcodes.iter_mut().filter(|set| set.len() > 1) {
                    for k in actual_opcodes.keys() {
                        set.remove(k);
                    }
                }
            }
        }

        if program_started {
            let command = parse_command(first_line);
            let method_index = actual_opcodes
                .get(&(command[0] as u8))
                .expect("Opcode mapping");
            let method = methods[*method_index];

            method(&command, &mut registers);
        }
    }

    println!("Samples matching 3: {}", count_with_three);
    println!("Register zero: {}", registers[0]);
}

fn parse_registers(input: &str) -> Vec<i32> {
    lazy_static! {
        static ref reg_pattern: Regex =
            Regex::new(r"\[(\d+), (\d+), (\d+), (\d+)\]").expect("Register pattern");
    }

    let caps = reg_pattern
        .captures(input)
        .expect("You told me there'd be registers");

    vec![
        caps[1].parse().expect("first"),
        caps[2].parse().expect("second"),
        caps[3].parse().expect("third"),
        caps[4].parse().expect("fourth"),
    ]
}

fn parse_command(input: &str) -> Vec<i32> {
    input
        .split_whitespace()
        .map(|x| x.parse().expect("command number"))
        .collect()
}

fn reg_equal(first: &Vec<i32>, second: &Vec<i32>) -> bool {
    if first.len() != second.len() {
        println!("Debug: register lengths differ");
        return false;
    }
    for i in 0..4 {
        if first[i] != second[i] {
            return false;
        }
    }
    return true;
}

fn exec_and_check(
    method: &Fn(&Vec<i32>, &mut Vec<i32>),
    command: &Vec<i32>,
    regs: &Vec<i32>,
    expected: &Vec<i32>,
) -> bool {
    let mut temp_reg = reg_copy(&regs);

    method(command, &mut temp_reg);
    return reg_equal(&temp_reg, &expected);
}

fn reg_copy(original: &Vec<i32>) -> Vec<i32> {
    return vec![original[0], original[1], original[2], original[3]];
}

fn addr(command: &Vec<i32>, regs: &mut Vec<i32>) {
    regs[command[3] as usize] = regs[command[1] as usize] + regs[command[2] as usize];
}
fn addi(command: &Vec<i32>, regs: &mut Vec<i32>) {
    regs[command[3] as usize] = regs[command[1] as usize] + command[2];
}

fn mulr(command: &Vec<i32>, regs: &mut Vec<i32>) {
    regs[command[3] as usize] = regs[command[1] as usize] * regs[command[2] as usize];
}
fn muli(command: &Vec<i32>, regs: &mut Vec<i32>) {
    regs[command[3] as usize] = regs[command[1] as usize] * command[2];
}

fn banr(command: &Vec<i32>, regs: &mut Vec<i32>) {
    regs[command[3] as usize] = regs[command[1] as usize] & regs[command[2] as usize];
}
fn bani(command: &Vec<i32>, regs: &mut Vec<i32>) {
    regs[command[3] as usize] = regs[command[1] as usize] & command[2];
}

fn borr(command: &Vec<i32>, regs: &mut Vec<i32>) {
    regs[command[3] as usize] = regs[command[1] as usize] | regs[command[2] as usize];
}
fn bori(command: &Vec<i32>, regs: &mut Vec<i32>) {
    regs[command[3] as usize] = regs[command[1] as usize] | command[2];
}

fn setr(command: &Vec<i32>, regs: &mut Vec<i32>) {
    regs[command[3] as usize] = regs[command[1] as usize];
}
fn seti(command: &Vec<i32>, regs: &mut Vec<i32>) {
    regs[command[3] as usize] = command[1];
}

fn gtir(command: &Vec<i32>, regs: &mut Vec<i32>) {
    if command[1] > regs[command[2] as usize] {
        regs[command[3] as usize] = 1;
    } else {
        regs[command[3] as usize] = 0;
    }
}
fn gtri(command: &Vec<i32>, regs: &mut Vec<i32>) {
    if regs[command[1] as usize] > command[2] {
        regs[command[3] as usize] = 1;
    } else {
        regs[command[3] as usize] = 0;
    }
}
fn gtrr(command: &Vec<i32>, regs: &mut Vec<i32>) {
    if regs[command[1] as usize] > regs[command[2] as usize] {
        regs[command[3] as usize] = 1;
    } else {
        regs[command[3] as usize] = 0;
    }
}

fn eqir(command: &Vec<i32>, regs: &mut Vec<i32>) {
    regs[command[3] as usize] = if command[1] == regs[command[2] as usize] {
        1
    } else {
        0
    }
}
fn eqri(command: &Vec<i32>, regs: &mut Vec<i32>) {
    regs[command[3] as usize] = if regs[command[1] as usize] == command[2] {
        1
    } else {
        0
    }
}
fn eqrr(command: &Vec<i32>, regs: &mut Vec<i32>) {
    regs[command[3] as usize] = if regs[command[1] as usize] == regs[command[2] as usize] {
        1
    } else {
        0
    }
}
