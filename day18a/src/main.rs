use std::io::Read;

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

    let mut map = vec![false; size.iter().map(|&x| x as usize).product::<usize>()];

    for &cube in &cubes {
        map[cube.iter().zip(&size).fold(0, |acc, (&c, &m)| {
            acc * m as usize + c as usize
        })] = true;
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
            !map[cube.iter().zip(d).zip(&size).fold(0, |acc, ((&c, d), &m)| {
                let r = c + d;
                acc * m as usize + r as usize
            })]
        }).count()
    }).sum();

    println!("{}", area);
}
