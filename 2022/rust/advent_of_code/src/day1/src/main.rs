use std::fs;

fn main() {
    let input = fs::read_to_string("src/day1/input/input.txt")
        .expect("Cannot open input file");

    let elf_inventories = get_elfs_inventory(input.trim());
    let mut elf_calories = elf_inventories
        .iter()
        .map(|&elf_inventory| get_elf_calories(elf_inventory))
        .collect::<Vec<u32>>();

    // Part One
    println!("Part one: The elf with the maximum calories has: {} calories", elf_calories.iter().max().unwrap());

    // Part two
    // Reverse sort
    elf_calories.sort_by(|a, b| b.cmp(a));
    println!("Part two: The top three elfs together have {} calories", elf_calories[0..3].iter().sum::<u32>());


    /*
    Rust like:
     */
    let mut calories = input.trim().split("\n\n")
        .map(|elem| elem.split("\n")
            .map(|x| x.parse::<u32>().unwrap())
            .reduce(|acc, x| acc + x).unwrap())
        .collect::<Vec<u32>>();

    println!("Part one: The elf with the maximum calories has: {} calories", calories.iter().max().unwrap());
    // Reverse sort
    calories.sort_by(|a, b| b.cmp(a));
    println!("Part two: The top three elfs together have {} calories", calories[0..3].iter().sum::<u32>());
}

fn get_elfs_inventory(inventory: &str) -> Vec<&str> {
    inventory.split("\n\n").collect::<Vec<&str>>()
}

fn get_elf_calories(elf_inventory: &str) -> u32 {
    elf_inventory.split('\n').fold(0, |acc, x| {
        acc + x.parse::<u32>().unwrap()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    //noinspection SpellCheckingInspection
    #[test]
    fn split_full_inventory_into_elf_inventory() {
        let elfs = get_elfs_inventory("1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000");
        assert_eq!(elfs.len(), 5)
    }

    #[test]
    fn get_elf_total_calories() {
        let elf_calories = get_elf_calories("1000\n2000\n3000");
        assert_eq!(elf_calories, 6000)
    }
}
