use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let lines: Vec<_> = input.lines().collect();
    println!("{}", lines.chunks(3).map(|groups| {
        let mut in_sacks = [[false; 3]; 53];

        for (n, &sack) in groups.iter().enumerate() {
            for c in sack.chars() {
                let priority = if c.is_ascii_lowercase() {
                    c as u32 - 'a' as u32 + 1
                } else if c.is_ascii_uppercase() {
                    c as u32 - 'A' as u32 + 27
                } else {
                    panic!("{} was not ASCII letter", c)
                } as usize;
                in_sacks[priority][n] = true;
            }
        }

        in_sacks.into_iter().enumerate().find_map(|(priority, sacks)| {
            if sacks.into_iter().all(|x| x) { Some(priority) } else { None }
        }).unwrap()
    }).sum::<usize>());
}
