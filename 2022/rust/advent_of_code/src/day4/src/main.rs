use std::fs;

fn main() {
    let input = fs::read_to_string("src/day4/input/input.txt")
        .expect("Cannot open input file");

    let lines = input
        .trim()
        .split('\n')
        .map(|line| line.to_string())
        .collect::<Vec<String>>();

    let mut c = 0u32;
    for line in lines {
        let mut elf_sectors = line.split(',')
            .map(|elf_sectors| get_elf_sector(elf_sectors.to_string()))
            .collect::<Vec<Vec<u32>>>();

        // Order the sectors so the shorted one is always the first one. That's the one that could
        //  be contained in the other
        elf_sectors.sort_by_key(|a| a.len());

        // Check if the shorter list of sectors is contained in the larger one
        if elf_sectors[0].iter().all(|sec| elf_sectors[1].contains(sec)) {
            c += 1;
        }
    }

    println!("Part one: There are {} assignments that fully contains the other", c);
}

fn get_elf_sector(sec: String) -> Vec<u32> {
    let mut s = sec.split('-').map(|s| s.parse::<u32>().unwrap()).collect::<Vec<u32>>();
    s.sort();

    (s[0]..s[1] + 1).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn elf_sectors_are_calculates() {
        let elf_sector = get_elf_sector(String::from("2-4"));

        assert_eq!(elf_sector, vec![2, 3, 4])
    }
}
