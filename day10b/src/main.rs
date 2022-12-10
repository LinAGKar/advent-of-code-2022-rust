use std::io::Read;

fn do_cycle(cycle: i16, x: i16) {
    let pos = cycle % 40;
    print!("{}", if pos >= x - 1 && pos <= x + 1 { '█' } else { ' ' });
    if pos == 39 {
        println!();
    }
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut cycle = 0;
    let mut x = 1;
    for line in input.lines() {
        let prev_cycle = cycle;
        let prev_x = x;

        let mut words = line.split_whitespace();
        cycle += match words.next().unwrap() {
            "addx" => {
                x += words.next().unwrap().parse::<i16>().unwrap();
                2
            }
            "noop" => 1,
            _ => panic!(),
        };

        for i in prev_cycle..cycle {
            do_cycle(i, prev_x);
        }
    }
}
