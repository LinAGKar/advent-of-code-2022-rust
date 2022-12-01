use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut top = [0; 3];
    for calories in input.split("\n\n").map(|elf|
        elf.lines().map(|item| item.parse::<u32>().unwrap()).sum::<u32>()
    ) {
        let top_min = top.iter_mut().min().unwrap();
        if calories > *top_min {
            *top_min = calories;
        }
    }

    println!("{}", top.iter().sum::<u32>());
}
