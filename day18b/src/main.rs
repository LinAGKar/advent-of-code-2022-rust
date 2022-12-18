use std::io::Read;

#[derive(PartialEq, Clone)]
enum Tile {
    Lava,
    Air,
    Steam,
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let cubes: Vec<_> = input.lines().map(|line| {
        let mut coord = [0i8; 3];
        for (src, dst) in line.split(',').map(|num| num.parse::<i8>().unwrap()).zip(&mut coord) {
            assert!(src >= 0);
            *dst = src + 1;
        }
        coord
    }).collect();

    let mut size = [i8::MIN; 3];
    for cube in &cubes {
        for (size, &coord) in size.iter_mut().zip(cube) {
            *size = std::cmp::max(*size, coord + 2);
        }
    }

    let mut map = vec![Tile::Air; size.iter().map(|&x| x as usize).product::<usize>()];

    for &cube in &cubes {
        map[cube.iter().zip(&size).fold(0, |acc, (&c, &m)| {
            acc * m as usize + c as usize
        })] = Tile::Lava;
    }

    let mut queue = std::collections::VecDeque::new();
    queue.push_back([0i8; 3]);
    map[0] = Tile::Steam;
    while let Some(pos) = queue.pop_front() {
        'outer: for d in [
            [-1, 0, 0],
            [1, 0, 0],
            [0, -1, 0],
            [0, 1, 0],
            [0, 0, -1],
            [0, 0, 1],
        ] {
            let mut new_pos = [0; 3];
            for ((new_pos, d), pos) in new_pos.iter_mut().zip(d).zip(pos) {
                *new_pos = pos + d;
            }
            for (&p, &s) in new_pos.iter().zip(&size) {
                if p < 0 || p >= s {
                    continue 'outer;
                }
            }
            let coord = new_pos.iter().zip(&size).fold(0, |acc, (&p, &m)| {
                acc * m as usize + p as usize
            });
            if map[coord] == Tile::Air {
                map[coord] = Tile::Steam;
                queue.push_back(new_pos);
            }
        }
    }

    let area: usize = cubes.iter().map(|cube| {
        [
            [-1, 0, 0],
            [1, 0, 0],
            [0, -1, 0],
            [0, 1, 0],
            [0, 0, -1],
            [0, 0, 1],
        ].into_iter().filter(|&d| {
            map[cube.iter().zip(d).zip(&size).fold(0, |acc, ((&c, d), &m)| {
                let r = c + d;
                acc * m as usize + r as usize
            })] == Tile::Steam
        }).count()
    }).sum();

    println!("{}", area);
}
