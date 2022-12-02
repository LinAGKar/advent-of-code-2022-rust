use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    println!("{}", input.lines().map(|line| {
        let mut words = line.split_whitespace().map(|word| word.chars().next().unwrap() as u32);
        let opponent = words.next().unwrap() - 'A' as u32;
        let player = words.next().unwrap() - 'X' as u32;
        let outcome = (player + 4 - opponent) % 3;
        (player + 1) + outcome * 3
    }).sum::<u32>());
}
