use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    print!("{}", input.split("\n\n").map(|elf|
        elf.lines().map(|item| item.parse::<u32>().unwrap()).sum::<u32>()
    ).max().unwrap());
}
