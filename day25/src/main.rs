use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut sum = input.lines().map(|line| {
        line.chars().fold(0, |acc, c| {
            acc * 5 + match c {
                '=' => -2,
                '-' => -1,
                '0' => 0,
                '1' => 1,
                '2' => 2,
                _ => panic!(),
            }
        })
    }).sum::<i64>();

    let mut digits = ['0'; 27];

    for c in &mut digits {
        let x = sum + 2;
        let digit = x.rem_euclid(5) - 2;
        sum = x.div_euclid(5);
        *c = match digit {
            -2 => '=',
            -1 => '-',
            0 => '0',
            1 => '1',
            2 => '2',
            _ => panic!(),
        };
        if sum == 0 {
            break;
        }
    }

    for c in digits.into_iter().rev().skip_while(|&c| c == '0') {
        print!("{}", c);
    }
    println!();
}
