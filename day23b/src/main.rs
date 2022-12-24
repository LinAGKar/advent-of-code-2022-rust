use std::collections::HashMap;
use std::io::Read;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Tile {
    Air,
    Elf,
    Proposal,
    Contested,
}

fn add_pos(pos: [i16; 2], delta: [i16; 2]) -> [i16; 2] {
    [pos[0] + delta[0], pos[1] + delta[1]]
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut elves: Vec<_> = input.lines().enumerate().flat_map(|(y, row)| {
        row.chars().enumerate().filter_map(move |(x, c)| {
            if c == '#' {
                Some(([x as i16, y as i16], None))
            } else {
                None
            }
        })
    }).collect();

    let mut map = HashMap::new();

    for &(elf, _) in &elves {
        map.insert(elf, Tile::Elf);
    }

    let considerations = [
        ([0, -1], [[-1, -1], [0, -1], [1, -1]]),
        ([0, 1], [[-1, 1], [0, 1], [1, 1]]),
        ([-1, 0], [[-1, -1], [-1, 0], [-1, 1]]),
        ([1, 0], [[1, -1], [1, 0], [1, 1]]),
    ];

    for round in 0.. {
        for (elf, proposed_pos) in &mut elves {
            if [
                [-1, -1],
                [0, -1],
                [1, -1],
                [-1, 0],
                [1, 0],
                [-1, 1],
                [0, 1],
                [1, 1],
            ].into_iter().any(|d| {
                map.get(&add_pos(*elf, d)) == Some(&Tile::Elf)
            }) {
                if let Some(proposal) = (0..4).find_map(|i| {
                    let (proposal, adjacent) = &considerations[(round + i) % considerations.len()];
                    if !adjacent.iter().any(|&d| map.get(&add_pos(*elf, d)) == Some(&Tile::Elf)) {
                        Some(*proposal)
                    } else {
                        None
                    }
                }) {
                    let new_pos = add_pos(*elf, proposal);
                    *proposed_pos = Some(new_pos);

                    match map.entry(new_pos).or_insert(Tile::Air) {
                        t @ Tile::Air => { *t = Tile::Proposal; }
                        t @ Tile::Proposal => { *t = Tile::Contested; }
                        Tile::Contested => {}
                        Tile::Elf => panic!(),
                    }
                }
            }
        }

        let mut moved = false;

        for (elf, proposal) in &mut elves {
            if let &mut Some(proposed_pos) = proposal {
                match map.get_mut(&proposed_pos) {
                    Some(Tile::Air) => {}
                    Some(t @ Tile::Contested) => { *t = Tile::Air; }
                    Some(t @ Tile::Proposal) => {
                        *t = Tile::Elf;
                        map.insert(*elf, Tile::Air);
                        *elf = proposed_pos;
                        moved = true;

                    }
                    None | Some(Tile::Elf) => panic!(),
                }

                *proposal = None;
            }
        }

        if !moved {
            println!("{}", round + 1);
            break;
        }
    }
}
