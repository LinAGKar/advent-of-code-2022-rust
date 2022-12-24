use std::{io::Read, collections::{VecDeque, HashSet}};

#[derive(Debug, Clone, Copy, PartialEq)]
enum Tile {
    Wall,
    Air(u8),
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut blizzards = Vec::<(_, u8)>::new();

    let mut map: Vec<Vec<_>> = input.lines().enumerate().map(|(y, line)| {
        line.chars().enumerate().map(|(x, c)| match c {
            '#' => Tile::Wall,
            '.' => Tile::Air(0),
            '>' => {
                blizzards.push(([y, x], 0));
                Tile::Air(1)
            }
            'v' => {
                blizzards.push(([y, x], 1));
                Tile::Air(1)
            }
            '<' => {
                blizzards.push(([y, x], 2));
                Tile::Air(1)
            }
            '^' => {
                blizzards.push(([y, x], 3));
                Tile::Air(1)
            }
            _ => panic!(),
        }).collect()
    }).collect();

    let mut curr_time = 0;

    let start = [0, 1];
    let end = [map.len() as isize - 1, map.last().unwrap().len() as isize - 2];
    let mut queue = VecDeque::new();
    queue.push_back((0, start));
    let mut visited = HashSet::new();
    visited.insert((0, start));

    while let Some((time, pos)) = queue.pop_front() {
        if pos == end {
            println!("{}", time);
            break;
        }

        let new_time = time + 1;
        if new_time > curr_time {
            curr_time += 1;

            for (pos, direction) in &mut blizzards {
                if let Tile::Air(count) = &mut map[pos[0]][pos[1]] {
                    *count -= 1;
                } else {
                    panic!();
                }

                match *direction {
                    0 => {
                        pos[1] += 1;
                        if map[pos[0]][pos[1]] == Tile::Wall {
                            pos[1] = 1;
                        }
                    }

                    1 => {
                        pos[0] += 1;
                        if map[pos[0]][pos[1]] == Tile::Wall {
                            pos[0] = 1;
                        }
                    }

                    2 => {
                        pos[1] -= 1;
                        if map[pos[0]][pos[1]] == Tile::Wall {
                            pos[1] = map[pos[0]].len() - 2;
                        }
                    }

                    3 => {
                        pos[0] -= 1;
                        if map[pos[0]][pos[1]] == Tile::Wall {
                            pos[0] = map.len() - 2;
                        }
                    }

                    _ => panic!(),
                }

                if let Tile::Air(count) = &mut map[pos[0]][pos[1]] {
                    *count += 1;
                } else {
                    panic!();
                }
            }
        }

        for d in [
            [-1, 0],
            [1, 0],
            [0, -1],
            [0, 1],
            [0, 0],
        ] {
            let new_pos = [pos[0] + d[0], pos[1] + d[1]];
            if visited.contains(&(new_time, new_pos)) {
                continue;
            }
            if new_pos.iter().any(|&pos| pos < 0) {
                continue;
            }
            visited.insert((new_time, new_pos));
            match map.get(new_pos[0] as usize).and_then(|line| line.get(new_pos[1] as usize)) {
                Some(Tile::Air(0)) => {
                    queue.push_back((new_time, new_pos));
                }
                None | Some(Tile::Wall) | Some(Tile::Air(1..)) => {}
            }
        }
    }
}
