use std::cmp::Ordering::*;
use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut visited = std::collections::HashSet::new();
    let mut rope = [[0i16; 2]; 10];
    let rope_len = rope.len();
    visited.insert(*rope.last().unwrap());
    for line in input.lines() {
        let mut words = line.split_whitespace();
        let direction = words.next().unwrap();
        let distance: i16 = words.next().unwrap().parse().unwrap();

        for _ in 0..distance {
            match direction {
                "L" => rope[0][0] -= 1,
                "R" => rope[0][0] += 1,
                "D" => rope[0][1] -= 1,
                "U" => rope[0][1] += 1,
                _ => panic!(),
            }

            for i in 0..rope.len() - 1 {
                let (a, b) = rope.split_at_mut(i + 1);
                if b.first().unwrap().iter().zip(a.last().unwrap()).any(|(&t, &h)| t < h - 1 || t > h + 1) {
                    for (t, h) in b.first_mut().unwrap().iter_mut().zip(a.last().unwrap()) {
                        match (*t).cmp(h) {
                            Greater => *t -= 1,
                            Less => *t += 1,
                            Equal => {}
                        }
                    }

                    if i == rope_len - 2 {
                        visited.insert(*b.first().unwrap());
                    }
                }

            }
        }
    }

    println!("{}", visited.len());
}
