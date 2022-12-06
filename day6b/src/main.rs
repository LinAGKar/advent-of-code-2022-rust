use std::io::Read;

fn main() {
    let mut input = Vec::new();
    std::io::stdin().read_to_end(&mut input).unwrap();
    while input.last().map_or(false, |&c| (c as char).is_whitespace()) {
        input.pop();
    }

    const MARKER_SIZE: usize = 14;
    let mut occurrences = [0u16; 256];
    let mut different_letters = 0;
    println!("{}", input.iter().enumerate().find_map(|(n, &i)| {
        if n >= MARKER_SIZE {
            occurrences[input[n - MARKER_SIZE] as usize] -= 1;
            if occurrences[input[n - MARKER_SIZE] as usize] == 0 {
                different_letters -= 1;
            }
        }
        if occurrences[i as usize] == 0 {
            different_letters += 1;
        }
        occurrences[i as usize] += 1;
        if different_letters == MARKER_SIZE {
            Some(n + 1)
        } else {
            None
        }
    }).unwrap());
}
