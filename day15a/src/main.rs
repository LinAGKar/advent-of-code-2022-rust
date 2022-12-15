use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    // Parse sensors
    let sensors: Vec<_> = input.lines().map(|line| {
        let mut parts = line.split_whitespace().filter_map(|word| {
            word.trim_end_matches(&[',', ':']).split("=").nth(1).map(|num| num.parse::<i32>().unwrap())
        });
        ([parts.next().unwrap(), parts.next().unwrap()], [parts.next().unwrap(), parts.next().unwrap()])
    }).collect();

    const LINE_Y: i32 = 2000000;

    // Find covered ranges on LINE_Y
    let mut ranges: Vec<_> = sensors.iter().filter_map(|&(sensor, beacon)| {
        let radius = sensor.iter().zip(&beacon).map(|(&a, &b)| (a - b).abs()).sum::<i32>();
        let line_radius = radius - (sensor[1] - LINE_Y).abs();
        if line_radius >= 0 {
            Some((sensor[0] - line_radius, sensor[0] + line_radius))
        } else {
            None
        }
    }).collect();

    // Find beacons on LINE_Y, so we can subtract them
    let mut beacons = Vec::with_capacity(ranges.len());
    for &(_, beacon) in &sensors {
        if beacon[1] == LINE_Y && !beacons.contains(&beacon[0]) {
            beacons.push(beacon[0]);
        }
    }

    // Merge overlapping ranges
    for i in (1..ranges.len()).rev() {
        let range = ranges[i];
        for other in ranges.iter_mut().take(i) {
            if range.0 <= other.1 && other.0 <= range.1 {
                *other = (std::cmp::min(range.0, other.0), std::cmp::max(range.1, other.1));
                ranges.pop();
                break;
            }
        }
    }

    println!("{}", ranges.iter().map(|(start, end)| end - start + 1).sum::<i32>() - beacons.len() as i32);
}
