use std::cmp::Ordering::*;
use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut visited = std::collections::HashSet::new();
    let mut head = [0; 2];
    let mut tail = [0; 2];
    visited.insert(tail);
    for line in input.lines() {
        let mut words = line.split_whitespace();
        let direction = words.next().unwrap();
        let distance: i16 = words.next().unwrap().parse().unwrap();

        match direction {
            "L" => head[0] -= distance,
            "R" => head[0] += distance,
            "D" => head[1] -= distance,
            "U" => head[1] += distance,
            _ => panic!(),
        }

        while tail.iter().zip(&head).any(|(&t, &h)| t < h - 1 || t > h + 1) {
            for (t, h) in tail.iter_mut().zip(&head) {
                match (*t).cmp(h) {
                    Greater => *t -= 1,
                    Less => *t += 1,
                    Equal => {}
                }
            }

            visited.insert(tail);
        }
    }

    println!("{}", visited.len());
}
