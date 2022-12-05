use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut lines = input.lines();

    let stack_lines: Vec<_> = (&mut lines).take_while(|line| !line.chars().nth(1).unwrap().is_ascii_digit()).collect();
    let mut stacks = Vec::with_capacity(9);
    for line in stack_lines.into_iter().rev() {
        for (n, c) in line.chars().skip(1).step_by(4).enumerate() {
            if c.is_ascii_alphabetic() {
                while stacks.len() <= n {
                    stacks.push(Vec::with_capacity(20));
                }
                stacks[n].push(c);
            }
        }
    }

    for line in lines.skip(1) {
        let mut nums = line.split_whitespace().skip(1).step_by(2).map(|x| x.parse::<usize>().unwrap());
        let count = nums.next().unwrap();
        let src = nums.next().unwrap() - 1;
        let dst = nums.next().unwrap() - 1;

        for _ in 0..count {
            let val = stacks[src].pop().unwrap();
            stacks[dst].push(val);
        }
    }

    println!("{}", stacks.into_iter().map(|x| *x.last().unwrap()).collect::<String>());
}
