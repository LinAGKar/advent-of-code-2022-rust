use std::io::Read;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Tile {
    Wall,
    Air,
    Outside,
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut lines = input.lines();
    let map: Vec<Vec<_>> = lines.by_ref().take_while(|&line| !line.is_empty()).map(|line| {
        line.chars().map(|c| match c {
            ' ' => Tile::Outside,
            '#' => Tile::Wall,
            '.' => Tile::Air,
            _ => panic!(),
        }).collect()
    }).collect();

    let mut pos = [0, map[0].iter().enumerate().find_map(|(n, tile)| match tile {
        Tile::Air => Some(n),
        _ => None,
    }).unwrap() as isize];

    let mut direction = 0isize;

    let path = lines.next().unwrap();

    let mut prev = 0;
    for (i, c) in path.match_indices(|c: char| !c.is_ascii_digit()).chain([(path.len(), "")]) {
        if i > prev {
            for _ in 0..path[prev..i].parse::<u8>().unwrap() {
                let mut new_pos = pos;
                match direction {
                    0 => {
                        new_pos[1] += 1;
                        if new_pos[1] as usize >= map[new_pos[0] as usize].len() ||
                           map[new_pos[0] as usize][new_pos[1] as usize] == Tile::Outside {
                            new_pos[1] = map[new_pos[0] as usize].iter().enumerate().find_map(|(n, &t)|
                                if t != Tile::Outside {
                                    Some(n)
                                } else {
                                    None
                                }
                            ).unwrap() as isize;
                        }
                    }

                    1 => {
                        new_pos[0] += 1;
                        if new_pos[0] as usize >= map.len() ||
                           new_pos[1] as usize >= map[new_pos[0] as usize].len() ||
                           map[new_pos[0] as usize][new_pos[1] as usize] == Tile::Outside {
                            new_pos[0] = map.iter().enumerate().find_map(|(n, row)|
                                if row.len() > new_pos[1] as usize && row[new_pos[1] as usize] != Tile::Outside {
                                    Some(n)
                                } else {
                                    None
                                }
                            ).unwrap() as isize;
                        }
                    }

                    2 => {
                        new_pos[1] -= 1;
                        if new_pos[1] < 0 || map[new_pos[0] as usize][new_pos[1] as usize] == Tile::Outside {
                            new_pos[1] = map[new_pos[0] as usize].iter().enumerate().rev().find_map(|(n, &t)|
                                if t != Tile::Outside {
                                    Some(n)
                                } else {
                                    None
                                }
                            ).unwrap() as isize;
                        }
                    }

                    3 => {
                        new_pos[0] -= 1;
                        if new_pos[0] < 0 || new_pos[1] as usize >= map[new_pos[0] as usize].len() ||
                           map[new_pos[0] as usize][new_pos[1] as usize] == Tile::Outside {
                            new_pos[0] = map.iter().enumerate().rev().find_map(|(n, row)|
                                if row.len() > new_pos[1] as usize && row[new_pos[1] as usize] != Tile::Outside {
                                    Some(n)
                                } else {
                                    None
                                }
                            ).unwrap() as isize;
                        }
                    }
                    _ => panic!(),
                }
                if map[new_pos[0] as usize][new_pos[1] as usize] == Tile::Wall {
                    break;
                }
                pos = new_pos;
            }
        }
        prev = i + 1;
        match c {
            "L" => { direction = (direction - 1).rem_euclid(4); }
            "R" => { direction = (direction + 1).rem_euclid(4); }
            "" => {}
            _ => panic!(),
        }
    }

    println!("{}", 1000 * (pos[0] + 1) + 4 * (pos[1] + 1) + direction);
}
