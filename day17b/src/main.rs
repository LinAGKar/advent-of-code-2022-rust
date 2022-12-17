fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let mut directions = input.trim().chars().enumerate().cycle();

    const WIDTH: usize = 7;
    const END: u64 = 1000000000000;

    let blocks = [
        vec![(0, 0), (0, 1), (0, 2), (0, 3)],
        vec![(0, 1), (1, 0), (1, 1), (1, 2), (2, 1)],
        vec![(0, 0), (0, 1), (0, 2), (1, 2), (2, 2)],
        vec![(0, 0), (1, 0), (2, 0), (3, 0)],
        vec![(0, 0), (0, 1), (1, 0), (1, 1)],
    ];

    let mut map = vec![[false; WIDTH]; 0];
    let mut settled = 0u64;
    let mut block = 0;
    let mut pos = (3isize, 2isize);

    let mut cycle: Option<(u64, usize, usize, Vec<usize>)> = None;

    loop {
        let (dir_index, direction) = directions.next().unwrap();
        let new_x = match direction {
            '<' => pos.1 - 1,
            '>' => pos.1 + 1,
            _ => panic!(),
        };

        if new_x >= 0 && blocks[block].iter().all(|&(y, x)| {
            let (abs_y, abs_x) = (pos.0 as usize + y, new_x as usize + x);
            abs_x < WIDTH && (abs_y >= map.len() || !map[abs_y][abs_x])
        }) {
            pos.1 = new_x;
        }

        let new_y = pos.0 - 1;
        if new_y < 0 || blocks[block].iter().any(|&(y, x)| {
            let (abs_y, abs_x) = (new_y as usize + y, pos.1 as usize + x);
            abs_y < map.len() && map[abs_y][abs_x]
        }) {
            for &(y, x) in &blocks[block] {
                let (abs_y, abs_x) = (pos.0 as usize + y, pos.1 as usize + x);
                while abs_y >= map.len() {
                    map.push([false; WIDTH]);
                }
                map[abs_y][abs_x] = true;
            }

            pos = (map.len() as isize + 3, 2);
            block = (block + 1) % blocks.len();
            settled += 1;

            if let Some((_, _, _, heights)) = &mut cycle {
                heights.push(map.len() - heights[0]);
            }

            if settled == END {
                println!("{}", map.len());
                break;
            } else if map.last().map_or(false, |top| top.iter().all(|&x| x)) {
                if let Some((start_settled, start_block, start_dir_index, heights)) = &mut cycle {
                    if block == *start_block && dir_index == *start_dir_index {
                        let cycle_height_len = (map.len() - heights[0]) as u64;
                        let cycle_settled_len = settled - *start_settled;
                        let remaining_settled = END - settled;
                        let remaining_cycles = remaining_settled / cycle_settled_len;
                        let remaining_remainder = remaining_settled % cycle_settled_len;
                        println!(
                            "{}",
                            map.len() as u64 +
                            remaining_cycles * cycle_height_len +
                            heights[remaining_remainder as usize] as u64,
                        );
                        break;
                    }
                } else {
                    cycle = Some((settled, block, dir_index, vec![map.len()]));
                }
            }
        } else {
            pos.0 = new_y;
        }
    }
}
