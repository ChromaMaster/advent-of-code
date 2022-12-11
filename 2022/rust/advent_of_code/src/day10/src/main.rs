// #![deny(unused)]

use std::fs;

#[derive(Debug)]
enum Instr {
    Add { cycles: u32, value: i32 },
    Nop { cycles: u32 },
    Unknown(String),
}

impl From<&str> for Instr {
    fn from(input: &str) -> Self {
        let mut split = input.split(' ');
        let instr = split.next().unwrap();
        match instr {
            "addx" => Instr::Add { cycles: 2, value: split.next().unwrap().parse::<i32>().unwrap() },
            "noop" => Instr::Nop { cycles: 1 },
            _ => Instr::Unknown(instr.to_string())
        }
    }
}

impl Instr {
    pub fn get_cpi(&self) -> u32 {
        match *self {
            Instr::Add { cycles, .. } => cycles,
            Instr::Nop { cycles } => cycles,
            _ => 0,
        }
    }
}

struct CPU {
    register: i32,

    running_instruction: Option<Instr>,
    busy_for: u32,

    clock_ticks: u64,

}

impl CPU {
    pub fn new() -> Self {
        Self { register: 1, running_instruction: None, busy_for: 0, clock_ticks: 0 }
    }

    pub fn exec(&mut self, instr: Instr) -> Result<(), String> {
        if self.running_instruction.is_some() {
            return Err(String::from("CPU busy"));
        }

        self.busy_for = instr.get_cpi();
        self.running_instruction = Some(instr);

        Ok(())
    }

    pub fn tick(&mut self) {
        self.clock_ticks += 1;
        if self.busy_for > 0 { self.busy_for -= 1 }
    }

    pub fn process_pipeline(&mut self) {
        if self.running_instruction.is_none() {
            return;
        }

        let instr = self.running_instruction.as_ref().unwrap();

        if self.busy_for == 0 {
            match instr {
                Instr::Add { value, .. } => self.register += value,
                Instr::Nop { .. } => {}
                _ => {}
            }
            self.running_instruction = None;
        }
    }

    pub fn is_busy(&self) -> bool {
        self.running_instruction.is_some()
    }
}


fn main() {
    let input = fs::read_to_string("src/day10/input/input.txt")
        .expect("Cannot open input file");

    let lines = input.trim().split('\n').collect::<Vec<&str>>();

    let mut cpu = CPU::new();
    let mut signal_strength = 0u64;
    let mut cycles_until_measurement = 20u32;
    for &line in lines.iter() {
        let _ = cpu.exec(Instr::from(line));
        while cpu.is_busy() {
            cpu.tick();

            cycles_until_measurement -= 1;

            if cycles_until_measurement == 0 {
                cycles_until_measurement = 40;
                signal_strength += (cpu.clock_ticks as i32 * cpu.register) as u64;
            }

            if cpu.clock_ticks == 220 {
                break;
            }
            cpu.process_pipeline();
        }
    }

    println!("Part one: Signal strength is: {}", signal_strength);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cpu_add_operation_takes_3_cycles_to_finish() {
        let mut cpu = CPU::new();

        assert!(cpu.exec(Instr::from("addx 3")).is_ok());

        assert_eq!(cpu.register, 1);
        cpu.tick();
        cpu.process_pipeline();
        assert_eq!(cpu.register, 1);
        cpu.tick();
        cpu.process_pipeline();
        assert_eq!(cpu.register, 4);
    }

    #[test]
    fn cpu_cannot_process_instruction_while_its_busy() {
        let mut cpu = CPU::new();

        assert!(cpu.exec(Instr::from("nop")).is_ok());
        assert!(cpu.exec(Instr::from("nop")).is_err());
    }
}
