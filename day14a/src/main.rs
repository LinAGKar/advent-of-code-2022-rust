use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut size = [501, 0];

    for nums in input.split(&[' ', '-', '>', '\n']).filter(|x| !x.is_empty()) {
        for (pos, size) in nums.split(',').map(|num| num.parse::<usize>().unwrap()).zip(&mut size) {
            *size = std::cmp::max(pos + 1, *size);
        }
    }

    size[0] += 1;

    let mut map = vec![vec![false; size[1]]; size[0]];

    for line in input.lines() {
        let mut prev: Option<[usize; 2]> = None;
        for part in line.split(" -> ") {
            let mut curr = [0; 2];
            for (src, dst) in part.split(',').map(|num| num.parse::<usize>().unwrap()).zip(&mut curr) {
                *dst = src;
            }

            if let Some(prev) = prev {
                if curr[0] > prev[0] {
                    for x in prev[0]..=curr[0] {
                        map[x][prev[1]] = true;
                    }
                } else if prev[0] > curr[0] {
                    for x in curr[0]..=prev[0] {
                        map[x][prev[1]] = true;
                    }
                } else if curr[1] > prev[1] {
                    for y in prev[1]..=curr[1] {
                        map[prev[0]][y] = true;
                    }
                } else if prev[1] > curr[1] {
                    for y in curr[1]..=prev[1] {
                        map[prev[0]][y] = true;
                    }
                }
            }

            prev = Some(curr);
        }
    }

    let mut settled = 0;

    'outer: loop {
        let mut pos = [500, 0];
        loop {
            let new_y = pos[1] + 1;
            if new_y >= size[1] {
                break 'outer;
            } else if !map[pos[0]][new_y] {
                pos = [pos[0], new_y];
            } else if !map[pos[0] - 1][new_y] {
                pos = [pos[0] - 1, new_y];
            } else if !map[pos[0] + 1][new_y] {
                pos = [pos[0] + 1, new_y];
            } else {
                map[pos[0]][pos[1]] = true;
                settled += 1;
                break;
            }
        }
    }

    println!("{}", settled);
}
