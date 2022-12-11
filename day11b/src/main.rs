use regex::Regex;
use std::collections::VecDeque;
use std::io::Read;

enum Operation {
    Add(u64),
    Mult(u64),
    Square,
}

struct Monkey {
    items: VecDeque<u64>,
    op: Operation,
    divisor: u64,
    t: usize,
    f: usize,
    inspections: u64,
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let re = Regex::new(concat!(
        r"(?m)^Monkey (\d+):\n  Starting items: (\d+(?:, \d+)*)\n  Operation: new = old (?:\* (?:(\d+)|(old))|\+ ",
        r"(\d+))\n  Test: divisible by (\d+)\n    If true: throw to monkey (\d+)\n    If false: throw to monkey (\d+)$",
    )).unwrap();

    let mut monkeys: Vec<_> = re.captures_iter(&input).enumerate().map(|(n, m)| {
        assert_eq!(m[1].parse::<usize>().unwrap(), n);

        Monkey {
            items: m[2].split(", ").map(|num| num.parse().unwrap()).collect(),
            op: if let Some(factor) = m.get(3) {
                Operation::Mult(factor.as_str().parse().unwrap())
            } else if let Some(_) = m.get(4) {
                Operation::Square
            } else if let Some(term) = m.get(5) {
                 Operation::Add(term.as_str().parse().unwrap())
            } else {
                panic!()
            },
            divisor: m[6].parse().unwrap(),
            t: m[7].parse().unwrap(),
            f: m[8].parse().unwrap(),
            inspections: 0,
        }
    }).collect();
    let modulo = monkeys.iter().fold(1, |acc, monkey|
        if acc % monkey.divisor == 0 { acc } else { acc * monkey.divisor }
    );

    for _ in 0..10000 {
        for i in 0..monkeys.len() {
            while let Some(item) = monkeys[i].items.pop_front() {
                let monkey = &monkeys[i];
                let item = match monkey.op {
                    Operation::Mult(factor) => item * factor,
                    Operation::Square => item * item,
                    Operation::Add(term) => item + term,
                } % modulo;
                let target = if item % monkey.divisor == 0 { monkey.t } else { monkey.f };
                monkeys[i].inspections += 1;
                monkeys[target].items.push_back(item);
            }
        }
    }

    let mut top = [0; 2];
    for monkey in &monkeys {
        let i = top.iter_mut().min().unwrap();
        if monkey.inspections > *i {
            *i = monkey.inspections;
        }
    }
    println!("{}", top.iter().product::<u64>());
}
