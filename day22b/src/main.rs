use std::io::Read;

#[derive(Debug, PartialEq)]
enum Tile {
    Wall,
    Air,
    Outside,
}

#[derive(Debug, Clone, Copy)]
enum Turn {
    Straight,
    Concave,
    Convex,
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut lines = input.lines();
    let map: Vec<Vec<_>> = [Vec::new()]
            .into_iter().chain(lines.by_ref().take_while(|&line| !line.is_empty()).map(|line| {
        let y = line.chars().map(|c| match c {
            ' ' => Tile::Outside,
            '#' => Tile::Wall,
            '.' => Tile::Air,
            _ => panic!(),
        });
        let mut row = Vec::with_capacity(y.size_hint().1.unwrap());
        row.push(Tile::Outside);
        for t in y {
            row.push(t);
        }
        row
    })).collect();

    let mut wraparounds = std::collections::HashMap::new();

    let walk_edge = |pos: [isize; 2], direction: [isize; 2], normal: [isize; 2]| {
        let new_pos = [pos[0] + direction[0], pos[1] + direction[1]];
        match (
            map.get(new_pos[0] as usize).and_then(|row| row.get(new_pos[1] as usize)),
            map.get((new_pos[0] + normal[0]) as usize).and_then(|row| row.get((new_pos[1] + normal[1]) as usize)),
        ) {
            (Some(Tile::Air | Tile::Wall), None | Some(Tile::Outside)) => (new_pos, direction, normal, Turn::Straight),
            (None | Some(Tile::Outside), None | Some(Tile::Outside)) =>
                (pos, [-normal[0], -normal[1]], direction, Turn::Convex),
            (Some(Tile::Air | Tile::Wall), Some(Tile::Air | Tile::Wall)) =>
                (pos, normal, [-direction[0], -direction[1]], Turn::Concave),
            (None | Some(Tile::Outside), Some(Tile::Air | Tile::Wall)) => panic!(),
        }
    };

    let normal_to_direction = |normal: [isize; 2]| {
        match normal {
            [0, 1] => 0,
            [1, 0] => 1,
            [0, -1] => 2,
            [-1, 0] => 3,
            _ => panic!(),
        }
    };

    let itou = |x: [isize; 2]| {
        [x[0] as usize, x[1] as usize]
    };

    for y in 0.. {
        if 50 * (y + 1) > map.len() {
            break;
        }

        for x in 0.. {
            let (y, x) = (50 * (y + 1), 50 * (x + 1));
            let mut tiles = [None; 4];
            for ((y, x), tile) in [
                (y, x),
                (y, x + 1),
                (y + 1, x),
                (y + 1, x + 1),
            ].into_iter().zip(&mut tiles) {
                *tile = map.get(y).and_then(|row| row.get(x));
            }
            let (y, x) = (y as isize, x as isize);
            let (mut positions, mut directions) = match tiles {
                [
                    Some(Tile::Air | Tile::Wall), Some(Tile::Air | Tile::Wall),
                    Some(Tile::Air | Tile::Wall), None | Some(Tile::Outside),
                ] => {
                    ([[y + 1, x], [y, x + 1]], [[1, 0], [0, 1]])
                }
                [
                    Some(Tile::Air | Tile::Wall), Some(Tile::Air | Tile::Wall),
                    None | Some(Tile::Outside), Some(Tile::Air | Tile::Wall),
                ] => {
                    ([[y, x], [y + 1, x + 1]], [[0, -1], [1, 0]])
                }
                [
                    Some(Tile::Air | Tile::Wall), None | Some(Tile::Outside),
                    Some(Tile::Air | Tile::Wall), Some(Tile::Air | Tile::Wall),
                ] => {
                    ([[y + 1, x + 1], [y, x]], [[0, 1], [-1, 0]])
                }
                [
                    None | Some(Tile::Outside), Some(Tile::Air | Tile::Wall),
                    Some(Tile::Air | Tile::Wall), Some(Tile::Air | Tile::Wall),
                ] => {
                    ([[y, x + 1], [y + 1, x]], [[-1, 0], [0, -1]])
                }
                [_, None, _, None] => { break; }
                _ => { continue; }
            };

            let mut normals = [
                [-directions[0][1], directions[0][0]],
                [directions[1][1],- directions[1][0]],
            ];

            loop {
                let mut dirs = [0; 2];
                for (dir, &normal) in dirs.iter_mut().zip(&normals) {
                    *dir = normal_to_direction(normal);
                }
                let mut outside_pos = [[0; 2]; 2];
                for ((outside_pos, &normal), &pos) in outside_pos.iter_mut().zip(&normals).zip(&positions) {
                    *outside_pos = [pos[0] + normal[0], pos[1] + normal[1]];
                }
                wraparounds.insert((itou(outside_pos[0]), dirs[0]), (itou(positions[1]), (dirs[1] + 2) % 4));
                wraparounds.insert((itou(outside_pos[1]), dirs[1]), (itou(positions[0]), (dirs[0] + 2) % 4));

                let mut turns = [Turn::Straight; 2];
                for (((pos, direction), normal), turn) in
                        positions.iter_mut().zip(&mut directions).zip(&mut normals).zip(&mut turns) {
                    let (new_pos, new_dir, new_normal, new_turn) = walk_edge(*pos, *direction, *normal);
                    *pos = new_pos;
                    *direction = new_dir;
                    *normal = new_normal;
                    *turn = new_turn;
                }

                match turns {
                    [Turn::Concave, Turn::Concave] => panic!(),
                    [Turn::Convex, Turn::Convex] => { break; },
                    _ => {}
                }
            }
        }
    }

    'outer: for _ in 0..1 {
        let start_pos = [(1..).find(|&x| map[1][x] != Tile::Outside).unwrap() as isize, 1];
        let mut pos = start_pos;
        let mut dir = [0, -1];
        let mut normal = [-1, 0];

        while !wraparounds.contains_key(
            &([(pos[0] + normal[0]) as usize, (pos[1] + normal[1]) as usize], normal_to_direction(normal)),
        ) {
            let (new_pos, new_dir, new_normal, _) = walk_edge(pos, dir, normal);
            pos = new_pos;
            dir = new_dir;
            normal = new_normal;

            if pos == new_pos {
                break 'outer;
            }
        }


        let mut positions = [pos; 2];
        let mut directions = [dir, [-dir[0], -dir[1]]];
        let mut normals = [normal; 2];

        for i in 0..2 {
            let pos = &mut positions[i];
            let dir = &mut directions[i];
            let normal = &mut normals[i];

            while wraparounds.contains_key(
                &([(pos[0] + normal[0]) as usize, (pos[1] + normal[1]) as usize], normal_to_direction(*normal)),
            ) {
                let (new_pos, new_dir, new_normal, _) = walk_edge(*pos, *dir, *normal);
                *pos = new_pos;
                *dir = new_dir;
                *normal = new_normal;
            }
        }

        loop {
            let mut dirs = [0; 2];
            for (dir, &normal) in dirs.iter_mut().zip(&normals) {
                *dir = normal_to_direction(normal);
            }
            let mut outside_pos = [[0; 2]; 2];
            for ((outside_pos, &normal), &pos) in outside_pos.iter_mut().zip(&normals).zip(&positions) {
                *outside_pos = [pos[0] + normal[0], pos[1] + normal[1]];
            }

            if outside_pos.iter().zip(&dirs).any(|(&outside_pos, &dir)| {
                wraparounds.contains_key(&(itou(outside_pos), dir))
            }) {
                break;
            }

            wraparounds.insert((itou(outside_pos[0]), dirs[0]), (itou(positions[1]), (dirs[1] + 2) % 4));
            wraparounds.insert((itou(outside_pos[1]), dirs[1]), (itou(positions[0]), (dirs[0] + 2) % 4));

            for ((pos, direction), normal) in positions.iter_mut().zip(&mut directions).zip(&mut normals) {
                let (new_pos, new_dir, new_normal, _) = walk_edge(*pos, *direction, *normal);
                *pos = new_pos;
                *direction = new_dir;
                *normal = new_normal;
            }
        }
    }

    let mut pos = [1, map[1].iter().enumerate().find_map(|(n, tile)| match tile {
        Tile::Air => Some(n),
        _ => None,
    }).unwrap()];

    let mut direction = 0i8;

    let path = lines.next().unwrap();

    let mut prev = 0;
    for (i, c) in path.match_indices(|c: char| !c.is_ascii_digit()).chain([(path.len(), "")]) {
        if i > prev {
            for _ in 0..path[prev..i].parse::<u8>().unwrap() {
                let mut new_pos = pos;
                match direction {
                    0 => { new_pos[1] += 1; }
                    1 => { new_pos[0] += 1; }
                    2 => { new_pos[1] -= 1; }
                    3 => { new_pos[0] -= 1; }
                    _ => panic!(),
                }
                let mut new_direction = direction;
                match map.get(new_pos[0]).and_then(|row| row.get(new_pos[1])) {
                    None | Some(Tile::Outside) => {
                        let x = wraparounds[&(new_pos, direction)];
                        new_pos = x.0;
                        new_direction = x.1;
                    }
                    _ => {}
                }
                if map[new_pos[0]][new_pos[1]] == Tile::Wall {
                    break;
                }
                pos = new_pos;
                direction = new_direction;
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

    println!("{}", 1000 * pos[0] + 4 * pos[1] + direction as usize);
}
