use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut period_cycle = 0;
    let mut cycle = 0;
    let mut x = 1;
    let mut total = 0;
    for line in input.lines() {
        let prev_cycle = period_cycle;
        let prev_x = x;

        let mut words = line.split_whitespace();
        let inst_cycles = match words.next().unwrap() {
            "addx" => {
                x += words.next().unwrap().parse::<i16>().unwrap();
                2
            }
            "noop" => 1,
            _ => panic!(),
        };
        period_cycle += inst_cycles;
        cycle += inst_cycles;

        if period_cycle == 19 {
            total += x * (cycle + 1);
        } else if period_cycle == 20 && prev_cycle < 19 {
            total += prev_x * cycle;
        }
        if period_cycle >= 40 {
            period_cycle -= 40;
        }
    }

    println!("{}", total);
}
