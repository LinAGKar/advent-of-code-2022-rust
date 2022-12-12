use std::collections::{BTreeSet, HashMap};
use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut start = [0; 2];
    let mut goal = [0; 2];
    let map: Vec<Vec<_>> = input.lines().enumerate().map(|(y, line)| {
        line.chars().enumerate().map(|(x, c)| {
            (if c == 'S' {
                start = [y as i16, x as i16];
                'a'
            } else if c == 'E' {
                goal = [y as i16, x as i16];
                'z'
            } else {
                c
            }) as u8 - 'a' as u8
        }).collect()
    }).collect();

    let height = map.len() as i16;
    let width = map[0].len() as i16;

    let h = |pos: [i16; 2]| goal.iter().zip(&pos).map(|(a, b)| (a - b).abs()).sum::<i16>();

    let start_f_score = h(start);

    let mut f_scores = HashMap::new();
    f_scores.insert(start, start_f_score);

    let mut g_scores = HashMap::new();
    g_scores.insert(start, 0);

    let mut open_set = BTreeSet::new();
    open_set.insert((start_f_score, start));

    while let Some(&curr) = open_set.iter().next() {
        open_set.remove(&curr);

        let (f_score, pos) = curr;
        if pos == goal {
            println!("{}", f_score);
            break;
        }
        let g_score = g_scores[&pos];
        let old_height = map[pos[0] as usize][pos[1] as usize];

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
               map[new_pos[0] as usize][new_pos[1] as usize] > old_height + 1 {
                continue;
            }
            let tentative_g_score = g_score + 1;
            let old_g_score = g_scores.get(&new_pos).copied();
            if tentative_g_score < old_g_score.unwrap_or(i16::MAX) {
                if old_g_score.is_some() {
                    open_set.remove(&(f_scores[&new_pos], new_pos));
                }

                let new_f_score = tentative_g_score + h(new_pos);
                f_scores.insert(new_pos, new_f_score);
                g_scores.insert(new_pos, tentative_g_score);
                open_set.insert((new_f_score, new_pos));
            }
        }
    }
}
