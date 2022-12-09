use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let map: Vec<Vec<_>> = input.lines().map(|line|
        line.chars().map(|c| c.to_digit(10).unwrap() as i8).collect()
    ).collect();

    let height = map.len();
    let width = map[0].len();

    println!("{}", map
            .iter()
            .enumerate()
            .flat_map(|(y, row)| row.iter().enumerate().map(move |(x, &height)| (y, x, height)))
            .map(|(y, x, tree_height)| {
        let score = [
            (-1, 0),
            (1, 0),
            (0, -1),
            (0, 1),
        ].iter().map(|(dy, dx)| {
            let (mut y, mut x) = (y as isize, x as isize);
            let mut distance = 0;
            let mut altitude = -1;
            loop {
                y += dy;
                x += dx;
                if y < 0 || x < 0 || y as usize >= height || x as usize >= width {
                    break;
                }
                distance += 1;
                let this_height = map[y as usize][x as usize];
                if this_height > altitude {
                    // count += 1;
                    if this_height >= tree_height {
                        break;
                    }
                    altitude = this_height;
                }
            }
            distance
        }).product::<u32>();
        score
    }).max().unwrap());
}
