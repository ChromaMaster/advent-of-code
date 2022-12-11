#![deny(unused)]

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

    clock: u32,

}

impl CPU {
    pub fn new() -> Self {
        Self { register: 1, running_instruction: None, busy_for: 0, clock: 0 }
    }

    pub fn exec(&mut self, instr: Instr) -> Result<(), String> {
        if self.running_instruction.is_some() {
            return Err(String::from("CPU busy"));
        }

        self.busy_for = instr.get_cpi();
        self.running_instruction = Some(instr);

        Ok(())
    }

    pub fn process_pipeline(&mut self, clock: u32) {
        let ticks_elapsed = clock - self.clock;
        self.clock = clock;

        if self.running_instruction.is_none() {
            return;
        }

        let instr = self.running_instruction.as_ref().unwrap();
        if self.busy_for >= ticks_elapsed { self.busy_for -= ticks_elapsed; }
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

struct Pixel {
    lit: bool,
}

impl Pixel {
    pub fn new() -> Self {
        Self { lit: false }
    }

    pub fn lit(&mut self) {
        self.lit = true;
    }
}

struct CRT {
    width: usize,
    height: usize,

    pixels: Vec<Pixel>,
    current_row: usize,
}

impl CRT {
    pub fn new(width: usize, height: usize) -> Self {
        let mut crt = CRT { width, height, pixels: Vec::new(), current_row: 0 };

        for _ in 0..(crt.width * crt.height) {
            crt.pixels.push(Pixel::new());
        }

        crt
    }

    pub fn update_pixel(&mut self, clock: u32, sprite: i32) {
        if clock != 0 && clock % (self.width as u32) == 0 {
            self.current_row += 1;
        }

        if !(-1..=41).contains(&sprite) {
            return;
        }

        // FIX: Not quite a good solution. But the capital letters can be read.
        let m = self.current_row * self.width + sprite.unsigned_abs() as usize;

        if (m - 1..=m + 1).contains(&(clock as usize)) {
            self.pixels.get_mut(clock as usize).unwrap().lit();
        }
    }

    pub fn print(&self) {
        let pixels = self.pixels
            .iter()
            .map(|pixel| if pixel.lit { '#' } else { '.' })
            .collect::<Vec<char>>();

        let lines = pixels
            .chunks(self.width)
            .map(|l| l.iter().collect::<String>())
            .collect::<Vec<String>>();
        println!("{}", lines.join("\n"));
    }
}

fn main() {
    let input = fs::read_to_string("src/day10/input/input.txt")
        .expect("Cannot open input file");

    let lines = input.trim().split('\n').collect::<Vec<&str>>();

    let mut clock = 0u32;

    let mut cpu = CPU::new();
    let mut crt = CRT::new(40, 6);
    let mut signal_strength = 0u64;
    let mut cycles_until_measurement = 20u32;

    for &line in lines.iter() {
        let _ = cpu.exec(Instr::from(line));
        while cpu.is_busy() {
            crt.update_pixel(clock, cpu.register);
            clock += 1;

            cycles_until_measurement -= 1;
            if cycles_until_measurement == 0 && clock <= 220 {
                cycles_until_measurement = 40;
                signal_strength += (clock as i32 * cpu.register) as u64;
            }

            cpu.process_pipeline(clock);
        }
    }

    println!("Part one: Signal strength is: {}", signal_strength);
    println!("Part two: CRT:");
    crt.print();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cpu_add_operation_takes_3_cycles_to_finish() {
        let mut cpu = CPU::new();

        assert!(cpu.exec(Instr::from("addx 3")).is_ok());

        assert_eq!(cpu.register, 1);
        cpu.process_pipeline(1);
        assert_eq!(cpu.register, 1);
        cpu.process_pipeline(2);
        assert_eq!(cpu.register, 4);
    }

    #[test]
    fn cpu_cannot_process_instruction_while_its_busy() {
        let mut cpu = CPU::new();

        assert!(cpu.exec(Instr::from("nop")).is_ok());
        assert!(cpu.exec(Instr::from("nop")).is_err());
    }
}
