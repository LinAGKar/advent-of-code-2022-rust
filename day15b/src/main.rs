use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    const MAX_COORD: i32 = 4000000;

    // Rectangles (start corner and end corner, inclusive). Begin with one covering whole possible area
    let mut possibilities = vec![([-MAX_COORD, 0], [MAX_COORD, 2 * MAX_COORD])];
    let mut new_possibilities = Vec::new();
    for line in input.lines() {
        let mut parts = line.split_whitespace().filter_map(|word| {
            word.trim_end_matches(&[',', ':']).split("=").nth(1).map(|num| num.parse::<i32>().unwrap())
        });
        let sensor = [parts.next().unwrap(), parts.next().unwrap()];
        let beacon = [parts.next().unwrap(), parts.next().unwrap()];

        let radius = sensor.iter().zip(&beacon).map(|(&a, &b)| (a - b).abs()).sum::<i32>();

        // Coordinate system rotated by 45°. Only even coordinates in target system are integer in source system
        let center = [sensor[0] - sensor[1], sensor[0] + sensor[1]];
        let start = [center[0] - radius, center[1] - radius];
        let end = [center[0] + radius, center[1] + radius];

        for &p in &possibilities {
            let (p_start, p_end) = p;
            if !(0..2).all(|i| start[i] <= p_end[i] && p_start[i] <= end[i]) {
                new_possibilities.push(p);
            } else {
                if start[0] > p_start[0] {
                    new_possibilities.push((p_start, [start[0] - 1, p_end[1]]));
                }
                if p_end[0] > end[0] {
                    new_possibilities.push(([end[0] + 1, p_start[1]], p_end));
                }
                if start[1] > p_start[1] {
                    new_possibilities.push((
                        [std::cmp::max(start[0], p_start[0]), p_start[1]],
                        [std::cmp::min(end[0], p_end[0]), start[1] - 1],
                    ));
                }
                if p_end[1] > end[1] {
                    new_possibilities.push(([
                        std::cmp::max(start[0], p_start[0]), end[1] + 1],
                        [std::cmp::min(end[0], p_end[0]), p_end[1]],
                    ));
                }
            }
        }
        possibilities.clear();

        std::mem::swap(&mut possibilities, &mut new_possibilities);
    }

    // Assume there is a 1x1 rectangle somewhere within the allowed area
    for (start, end) in possibilities {
        if start == end && (start[0] + start[1]) % 2 == 0 {
            // Transform back into original coordinate system
            let pos = [(start[1] + start[0]) / 2, (start[1] - start[0]) / 2];
            if pos.iter().all(|&x| x >= 0 && x <= MAX_COORD) {
                println!("{}", pos[0] as i64 * MAX_COORD as i64 + pos[1] as i64);
                break;
            }
        }
    }
}
