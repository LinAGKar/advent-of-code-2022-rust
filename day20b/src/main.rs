use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut nums: Vec<_> = input.lines().enumerate().map(|(n, line)| {
        (line.parse::<i64>().unwrap() * 811589153, if n == 0 { 0 } else { n - 1 }, n + 1)
    }).collect();
    nums[0].1 = nums.len() - 1;
    nums.last_mut().unwrap().2 = 0;

    for _ in 0..10 {
        for i in 0..nums.len() {
            let (num, prev, next) = nums[i];
            let distance = num.rem_euclid((nums.len() - 1) as i64) as usize;

            if distance > 0 {
                let mut pos = i;
                if distance > nums.len() / 2 {
                    for _ in 0..nums.len() - distance {
                        pos = nums[pos].1;
                    }
                } else if distance > 0 {
                    for _ in 0..distance {
                        pos = nums[pos].2;
                    }
                }
                nums[next].1 = prev;
                nums[prev].2 = next;
                let prev = pos;
                let next = nums[prev].2;
                nums[prev].2 = i;
                nums[next].1 = i;
                nums[i].1 = prev;
                nums[i].2 = next;
            }
        }
    }

    let zero = nums.iter().enumerate().find_map(|(i, &(num, _, _))| {
        if num == 0 {
            Some(i)
        } else {
            None
        }
    }).unwrap();

    let result = [1000, 2000, 3000].into_iter().map(|i| {
        let mut pos = zero;
        for _ in 0..i % nums.len() {
            pos = nums[pos].2;
        }
        nums[pos].0
    }).sum::<i64>();

    println!("{}", result);
}
