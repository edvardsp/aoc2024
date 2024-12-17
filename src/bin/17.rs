use std::cmp::Ordering;

advent_of_code::solution!(17);

enum Instructions {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

impl Instructions {
    fn decode(opcode: usize) -> Option<Self> {
        match opcode {
            0 => Some(Instructions::Adv),
            1 => Some(Instructions::Bxl),
            2 => Some(Instructions::Bst),
            3 => Some(Instructions::Jnz),
            4 => Some(Instructions::Bxc),
            5 => Some(Instructions::Out),
            6 => Some(Instructions::Bdv),
            7 => Some(Instructions::Cdv),
            _ => None,
        }
    }
}

struct Cpu {
    areg: usize,
    breg: usize,
    creg: usize,
    ip: usize,
}

impl Cpu {
    fn reset(&mut self, areg: usize) {
        self.areg = areg;
        self.breg = 0;
        self.creg = 0;
        self.ip = 0;
    }

    fn combo(&self, operand: usize) -> usize {
        match operand {
            0..=3 => operand,
            4 => self.areg,
            5 => self.breg,
            6 => self.creg,
            _ => unreachable!("Invalid combo operand: {}", operand),
        }
    }

    fn execute(&mut self, program: &[usize]) -> Vec<usize> {
        let mut output = Vec::new();
        while let Some(opcode) = program.get(self.ip) {
            let operand = program[self.ip + 1];
            use Instructions::*;
            match Instructions::decode(*opcode).expect("Invalid instruction") {
                Adv => {
                    let num = self.areg;
                    let den = 2_usize.pow(self.combo(operand) as u32);
                    self.areg = num / den;
                    self.ip += 2;
                }
                Bxl => {
                    self.breg ^= operand;
                    self.ip += 2;
                }
                Bst => {
                    self.breg = self.combo(operand) % 8;
                    self.ip += 2;
                }
                Jnz => {
                    if self.areg == 0 {
                        self.ip += 2;
                    } else {
                        self.ip = operand;
                    }
                }
                Bxc => {
                    self.breg ^= self.creg;
                    self.ip += 2;
                }
                Out => {
                    let value = self.combo(operand) % 8;
                    output.push(value);
                    self.ip += 2;
                }
                Bdv => {
                    let num = self.areg;
                    let den = 2_usize.pow(self.combo(operand) as u32);
                    self.breg = num / den;
                    self.ip += 2;
                }
                Cdv => {
                    let num = self.areg;
                    let den = 2_usize.pow(self.combo(operand) as u32);
                    self.creg = num / den;
                    self.ip += 2;
                }
            }
        }
        output
    }
}

impl From<&str> for Cpu {
    fn from(value: &str) -> Self {
        let mut lines = value.lines();
        let areg_str = lines.next().unwrap().strip_prefix("Register A: ").unwrap();
        let breg_str = lines.next().unwrap().strip_prefix("Register B: ").unwrap();
        let creg_str = lines.next().unwrap().strip_prefix("Register C: ").unwrap();
        let areg = areg_str.parse().unwrap();
        let breg = breg_str.parse().unwrap();
        let creg = creg_str.parse().unwrap();
        let ip = 0;
        Self {
            areg,
            breg,
            creg,
            ip,
        }
    }
}

struct Input {
    cpu: Cpu,
    program: Vec<usize>,
}

impl From<&str> for Input {
    fn from(value: &str) -> Self {
        let (cpu_str, program_str) = value.split_once("\n\n").unwrap();
        let cpu: Cpu = cpu_str.into();
        let program: Vec<_> = program_str
            .strip_prefix("Program: ")
            .unwrap()
            .split(",")
            .map(|n| n.parse().unwrap())
            .collect();
        Self { cpu, program }
    }
}

pub fn part_one(input: &str) -> Option<String> {
    let Input { mut cpu, program } = input.into();
    let output: Vec<_> = cpu
        .execute(&program)
        .into_iter()
        .map(|n| n.to_string())
        .collect();
    Some(output.join(","))
}

pub fn part_two(input: &str) -> Option<usize> {
    let Input { mut cpu, program } = input.into();
    let mut lower = 0;
    for i in 0..program.len() - 1 {
        let offset = program.len() - i;
        let stride = 10_usize.pow(offset as u32 - 2); // -2 found by inspecting resulting output size
        let mut areg = lower;
        loop {
            cpu.reset(areg);
            let output = cpu.execute(&program);
            match output.len().cmp(&program.len()) {
                Ordering::Equal => {
                    if output[offset..] == program[offset..] {
                        break;
                    }
                    lower = areg;
                }
                Ordering::Less => lower = areg,
                Ordering::Greater => break,
            }
            areg += stride;
        }
    }

    for areg in lower.. {
        cpu.reset(areg);
        let output = cpu.execute(&program);
        if output == program {
            return Some(areg);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some("4,6,3,5,6,3,5,2,1,0".to_string()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(117440));
    }
}
