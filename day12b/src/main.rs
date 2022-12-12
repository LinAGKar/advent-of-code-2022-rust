use std::collections::{BinaryHeap, HashSet};
use std::cmp::Reverse;
use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut start = [0; 2];
    let map: Vec<Vec<_>> = input.lines().enumerate().map(|(y, line)| {
        line.chars().enumerate().map(|(x, c)| {
            (if c == 'S' {
                'a'
            } else if c == 'E' {
                start = [y as i16, x as i16];
                'z'
            } else {
                c
            }) as u8 - 'a' as u8
        }).collect()
    }).collect();

    let height = map.len() as i16;
    let width = map[0].len() as i16;

    let mut open_set = BinaryHeap::new();
    open_set.push((Reverse(0), 0, start));
    let mut visited = HashSet::new();

    while let Some((_, dist, pos)) = open_set.pop() {
        let old_height = map[pos[0] as usize][pos[1] as usize];
        if old_height == 0 {
            println!("{}", dist);
            break;
        }

        let new_dist = dist + 1;
        for delta in [
            [-1, 0],
            [1, 0],
            [0, -1],
            [0, 1],
        ] {
            let mut new_pos = [0; 2];
            for ((&old, &delta), new) in pos.iter().zip(&delta).zip(&mut new_pos) {
                *new = old + delta;
            }

            if new_pos.iter().any(|&x| x < 0) || new_pos[0] >= height || new_pos[1] >= width ||
               old_height > map[new_pos[0] as usize][new_pos[1] as usize] + 1 || visited.contains(&new_pos) {
                continue;
            }

            visited.insert(new_pos);
            open_set.push((Reverse(new_dist), new_dist, new_pos));
        }
    }
}
