fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let mut directions = input.trim().chars().cycle();

    const WIDTH: usize = 7;

    let blocks = [
        vec![(0, 0), (0, 1), (0, 2), (0, 3)],
        vec![(0, 1), (1, 0), (1, 1), (1, 2), (2, 1)],
        vec![(0, 0), (0, 1), (0, 2), (1, 2), (2, 2)],
        vec![(0, 0), (1, 0), (2, 0), (3, 0)],
        vec![(0, 0), (0, 1), (1, 0), (1, 1)],
    ];

    let mut map = vec![[false; WIDTH]; 0];
    let mut settled = 0u16;
    let mut block = 0;
    let mut pos = (3isize, 2isize);

    while settled < 2022 {
        let new_x = match directions.next() {
            Some('<') => pos.1 - 1,
            Some('>') => pos.1 + 1,
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
        } else {
            pos.0 = new_y;
        }
    }

    println!("{}", map.len());
}
