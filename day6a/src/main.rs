use std::io::Read;

fn main() {
    let mut input = Vec::new();
    std::io::stdin().read_to_end(&mut input).unwrap();
    while input.last().map_or(false, |&c| (c as char).is_whitespace()) {
        input.pop();
    }

    const MARKER_SIZE: usize = 4;
    println!("{}", input.windows(MARKER_SIZE).enumerate().find_map(|(n, window)| {
        if window.iter().enumerate().all(|(m, &a)| {
            window.iter().skip(m + 1).all(|&b| a != b)
        }) {
            Some(n + MARKER_SIZE)
        } else {
            None
        }
    }).unwrap());
}
