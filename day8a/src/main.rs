use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let map: Vec<Vec<_>> = input.lines().map(|line|
        line.chars().map(|c| c.to_digit(10).unwrap() as i8).collect()
    ).collect();
    let mut visible: Vec<Vec<_>> = map.iter().map(|row| row.iter().map(|_| false).collect()).collect();

    let height = map.len();
    let width = map[0].len();

    let mut visible_count = 0u16;
    for y in 0..height {
        let mut altitude = -1;
        for x in 0..width {
            let tree_height = map[y][x];
            if tree_height > altitude {
                altitude = tree_height;
                let this_visited = &mut visible[y][x];
                if !*this_visited {
                    *this_visited = true;
                    visible_count += 1;
                }
            }
        }

        let mut altitude = -1;
        for x in 0..width {
            let tree_height = map[y][width - 1 - x];
            if tree_height > altitude {
                altitude = tree_height;
                let this_visited = &mut visible[y][width - 1 - x];
                if !*this_visited {
                    *this_visited = true;
                    visible_count += 1;
                }
            }
        }
    }

    for x in 0..width {
        let mut altitude = -1;
        for y in 0..height {
            let tree_height = map[y][x];
            if tree_height > altitude {
                altitude = tree_height;
                let this_visited = &mut visible[y][x];
                if !*this_visited {
                    *this_visited = true;
                    visible_count += 1;
                }
            }
        }

        let mut altitude = -1;
        for y in 0..height {
            let tree_height = map[height - 1 - y][x];
            if tree_height > altitude {
                altitude = tree_height;
                let this_visited = &mut visible[height - 1 - y][x];
                if !*this_visited {
                    *this_visited = true;
                    visible_count += 1;
                }
            }
        }
    }

    println!("{}", visible_count);
}
