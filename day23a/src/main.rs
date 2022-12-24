use std::io::Read;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Tile {
    Air,
    Elf,
    Proposal,
    Contested,
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    const ROUNDS: usize = 10;
    let mut elves: Vec<_> = input.lines().enumerate().flat_map(|(y, row)| {
        row.chars().enumerate().filter_map(move |(x, c)| {
            if c == '#' {
                Some(([x + ROUNDS, y + ROUNDS], None))
            } else {
                None
            }
        })
    }).collect();

    let mut sizes = [0; 2];
    for (elf, _) in &elves {
        for (&coord, size) in elf.iter().zip(&mut sizes) {
            *size = std::cmp::max(*size, coord + ROUNDS + 1);
        }
    }
    let sizes = sizes;

    let mut map = vec![Tile::Air; sizes.iter().product()];

    let pos_to_index = |pos: [usize; 2]| {
        pos[0] * sizes[1] + pos[1]
    };

    for &(elf, _) in &elves {
        map[pos_to_index(elf)] = Tile::Elf;
    }

    let considerations = [
        ([1, 0], [[0, 0], [1, 0], [2, 0]]),
        ([1, 2], [[0, 2], [1, 2], [2, 2]]),
        ([0, 1], [[0, 0], [0, 1], [0, 2]]),
        ([2, 1], [[2, 0], [2, 1], [2, 2]]),
    ];

    let pos_d_to_index = |pos: [usize; 2], delta: [usize; 2]| {
        (pos[0] + delta[0] - 1) * sizes[1] + pos[1] + delta[1] - 1
    };

    for round in 0..ROUNDS {
        for (elf, proposed_pos) in &mut elves {
            if [
                [0, 0],
                [1, 0],
                [2, 0],
                [0, 1],
                [2, 1],
                [0, 2],
                [1, 2],
                [2, 2],
            ].into_iter().any(|d| {
                map[pos_d_to_index(*elf, d)] == Tile::Elf
            }) {
                if let Some(proposal) = (0..4).find_map(|i| {
                    let (proposal, adjacent) = &considerations[(round + i) % considerations.len()];
                    if !adjacent.iter().any(|&d| map[pos_d_to_index(*elf, d)] == Tile::Elf) {
                        Some(*proposal)
                    } else {
                        None
                    }
                }) {
                    let new_index = pos_d_to_index(*elf, proposal);
                    *proposed_pos = Some([elf[0] + proposal[0] - 1, elf[1] + proposal[1] - 1]);

                    match &mut map[new_index] {
                        t @ Tile::Air => { *t = Tile::Proposal }
                        t @ Tile::Proposal => { *t = Tile::Contested }
                        Tile::Contested => {}
                        Tile::Elf => panic!(),
                    }
                }
            }
        }

        for (elf, proposal) in &mut elves {
            if let &mut Some(proposed_pos) = proposal {
                match &mut map[pos_to_index(proposed_pos)] {
                    t @ (Tile::Contested | Tile::Air) => {
                        *t = Tile::Air;
                    }

                    t @ Tile::Proposal => {
                        *t = Tile::Elf;
                        map[pos_to_index(*elf)] = Tile::Air;
                        *elf = proposed_pos;
                    }

                    Tile::Elf => panic!(),
                }

                *proposal = None;
            }
        }
    }

    let mut min = [usize::MAX; 2];
    let mut max = [usize::MIN; 2];
    for (elf, _) in &elves {
        for ((&coord, min), max) in elf.iter().zip(&mut min).zip(&mut max) {
            *min = std::cmp::min(*min, coord);
            *max = std::cmp::max(*max, coord);
        }
    }

    println!("{}", (max[0] - min[0] + 1) * (max[1] - min[1] + 1) - elves.len());
}
