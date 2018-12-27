use std::fmt::*;

const CPU_SIZE: usize = 6;

pub fn solve() {
    println!("Day 19");

    let lines = adventlib::read_input_lines("day19input_optimized.txt");
    let mut lines_iter = lines.iter();

    let ip_declaration = lines_iter.next().expect("First line");
    let ip_reg: u8 = ip_declaration[4..]
        .parse()
        .expect("Line must be like '#ip <digit>'");
    let mut computer = WristComputer::new(ip_reg);
    let program: Vec<_> = lines_iter
        .map(|l| WristComputerInstruction::parse_str(l))
        .collect();

    computer.execute_program(&program);
    println!("Register 0 (part 1): {}", computer.regs[0]);

    let mut computer = WristComputer::new(ip_reg);
    computer.regs[0] = 1;
    computer.execute_program(&program);
    println!("Register 0 (part 2): {}", computer.regs[0]);
}

struct WristComputer {
    ip_reg: u8,
    regs: [i32; CPU_SIZE],
}

impl WristComputer {
    fn new(ip_reg: u8) -> WristComputer {
        let mut computer = WristComputer {
            ip_reg: ip_reg,
            regs: [0; CPU_SIZE],
        };
        computer.regs[ip_reg as usize] = -1; // since we naively increment
        return computer;
    }

    fn get_next_ip(&self) -> i32 {
        self.regs[self.ip_reg as usize] + 1
    }

    fn execute_program(&mut self, program: &Vec<WristComputerInstruction>) {
        let mut next_ip = 0;
        while next_ip >= 0 && next_ip < program.len() as i32 {
            self.execute(&program[next_ip as usize]);
            next_ip = self.get_next_ip();
        }
    }

    fn execute(&mut self, instr: &WristComputerInstruction) {
        self.regs[self.ip_reg as usize] = self.get_next_ip();

        match instr.method {
            "addr" => self.addr(&instr.args),
            "addi" => self.addi(&instr.args),
            "mulr" => self.mulr(&instr.args),
            "muli" => self.muli(&instr.args),
            "modr" => self.modr(&instr.args),
            "modi" => self.modi(&instr.args),
            "banr" => self.banr(&instr.args),
            "bani" => self.bani(&instr.args),
            "borr" => self.borr(&instr.args),
            "bori" => self.bori(&instr.args),
            "setr" => self.setr(&instr.args),
            "seti" => self.seti(&instr.args),
            "gtir" => self.gtir(&instr.args),
            "gtri" => self.gtri(&instr.args),
            "gtrr" => self.gtrr(&instr.args),
            "eqir" => self.eqir(&instr.args),
            "eqri" => self.eqri(&instr.args),
            "eqrr" => self.eqrr(&instr.args),
            any => println!("Invalid command: {}", any),
        }
    }

    fn addr(&mut self, args: &Vec<u8>) {
        self.regs[args[2] as usize] = self.regs[args[0] as usize] + self.regs[args[1] as usize];
    }

    fn addi(&mut self, args: &Vec<u8>) {
        self.regs[args[2] as usize] = self.regs[args[0] as usize] + args[1] as i32;
    }

    fn mulr(&mut self, args: &Vec<u8>) {
        self.regs[args[2] as usize] = self.regs[args[0] as usize] * self.regs[args[1] as usize];
    }

    fn muli(&mut self, args: &Vec<u8>) {
        self.regs[args[2] as usize] = self.regs[args[0] as usize] * args[1] as i32;
    }

    fn modr(&mut self, args: &Vec<u8>) {
        self.regs[args[2] as usize] = self.regs[args[0] as usize] % self.regs[args[1] as usize];
    }

    fn modi(&mut self, args: &Vec<u8>) {
        self.regs[args[2] as usize] = self.regs[args[0] as usize] % args[1] as i32;
    }

    fn banr(&mut self, args: &Vec<u8>) {
        self.regs[args[2] as usize] = self.regs[args[0] as usize] & self.regs[args[1] as usize];
    }

    fn bani(&mut self, args: &Vec<u8>) {
        self.regs[args[2] as usize] = self.regs[args[0] as usize] & args[1] as i32;
    }

    fn borr(&mut self, args: &Vec<u8>) {
        self.regs[args[2] as usize] = self.regs[args[0] as usize] | self.regs[args[1] as usize];
    }
    fn bori(&mut self, args: &Vec<u8>) {
        self.regs[args[2] as usize] = self.regs[args[0] as usize] | args[1] as i32;
    }

    fn setr(&mut self, args: &Vec<u8>) {
        self.regs[args[2] as usize] = self.regs[args[0] as usize];
    }
    fn seti(&mut self, args: &Vec<u8>) {
        self.regs[args[2] as usize] = args[0] as i32;
    }

    fn gtir(&mut self, args: &Vec<u8>) {
        if args[0] as i32 > self.regs[args[1] as usize] {
            self.regs[args[2] as usize] = 1;
        } else {
            self.regs[args[2] as usize] = 0;
        }
    }
    fn gtri(&mut self, args: &Vec<u8>) {
        if self.regs[args[0] as usize] > args[1] as i32 {
            self.regs[args[2] as usize] = 1;
        } else {
            self.regs[args[2] as usize] = 0;
        }
    }
    fn gtrr(&mut self, args: &Vec<u8>) {
        if self.regs[args[0] as usize] > self.regs[args[1] as usize] {
            self.regs[args[2] as usize] = 1;
        } else {
            self.regs[args[2] as usize] = 0;
        }
    }

    fn eqir(&mut self, args: &Vec<u8>) {
        self.regs[args[2] as usize] = if args[0] as i32 == self.regs[args[1] as usize] {
            1
        } else {
            0
        }
    }
    fn eqri(&mut self, args: &Vec<u8>) {
        self.regs[args[2] as usize] = if self.regs[args[0] as usize] == args[1] as i32 {
            1
        } else {
            0
        }
    }
    fn eqrr(&mut self, args: &Vec<u8>) {
        self.regs[args[2] as usize] = if self.regs[args[0] as usize] == self.regs[args[1] as usize]
        {
            1
        } else {
            0
        }
    }
}

impl Display for WristComputer {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "ip={} {:?}", self.regs[self.ip_reg as usize], self.regs)
    }
}

struct WristComputerInstruction<'a> {
    method: &'a str,
    args: Vec<u8>,
}

impl<'a> WristComputerInstruction<'a> {
    fn parse_str(line: &'a str) -> WristComputerInstruction<'a> {
        let args: Vec<u8> = line[5..]
            .split_whitespace()
            .map(|x| x.parse::<u8>().unwrap())
            .collect();
        WristComputerInstruction {
            method: &line[0..4],
            args: args,
        }
    }
}
