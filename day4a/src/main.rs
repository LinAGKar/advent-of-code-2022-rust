use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    println!("{}", input.lines().filter(|&line| {
        let mut elves = line.split(',').map(|elf| {
            let mut limits = elf.split('-').map(|num| num.parse::<u8>().unwrap());
            [limits.next().unwrap(), limits.next().unwrap()]
        });
        let elves = [elves.next().unwrap(), elves.next().unwrap()];
        (elves[0][0] <= elves[1][0] && elves[0][1] >= elves[1][1]) ||
            (elves[1][0] <= elves[0][0] && elves[1][1] >= elves[0][1])
    }).count());
}
