use std::io::Read;

fn find_max(
    costs: &[[u16; 3]; 4],
    inventory: [u16; 4],
    bots: [u16; 4],
    time: u16,
    level: u8,
    best: &mut u16,
) {
    const MAX_TIME: u16 = 32;
    let mut time_to_geode_bot = u16::MAX;
    let mut built_bot = false;

    'outer: for (n, (&cost, &bot)) in costs.iter().zip(&bots).enumerate().rev() {
        if n < costs[0].len() && costs.iter().all(|cost| bot >= cost[n]) {
            continue;
        }

        let mut needed_time = 0;
        for ((&cost, &inventory), &bots) in cost.iter().zip(&inventory).zip(&bots) {
            if inventory < cost {
                if bots == 0 {
                    continue 'outer;
                }
                let needed_resources = cost - inventory;
                needed_time = std::cmp::max(
                    needed_time,
                    needed_resources / bots + if needed_resources % bots != 0 { 1 } else { 0 },
                )
            }
        }
        needed_time += 1;

        if n == bots.len() - 1 {
            time_to_geode_bot = needed_time;
        } else if needed_time >= time_to_geode_bot {
            continue;
        }

        if time + needed_time >= MAX_TIME {
            continue;
        }

        let mut new_inventory = inventory;
        for (inv, &bots) in new_inventory.iter_mut().zip(&bots) {
            *inv += bots * needed_time;
        }
        for (inv, &cost) in new_inventory.iter_mut().zip(&cost) {
            *inv -= cost;
        }

        let mut new_bots = bots;
        new_bots[n] += 1;

        let remaining_time = MAX_TIME - (time + needed_time);
        if new_inventory.last().unwrap() +
           new_bots.last().unwrap() * remaining_time +
           remaining_time * (remaining_time - 1) / 2 <= *best {
            continue;
        }

        find_max(costs, new_inventory, new_bots, time + needed_time, level + 1, best);
        built_bot = true;
    }

    if !built_bot {
        *best = std::cmp::max(*best, inventory.last().unwrap() + bots.last().unwrap() * (MAX_TIME - time));
    }
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    println!("{}", input.lines().take(3).map(|line| {
        let mut words = line.split_whitespace();
        let costs: [[u16; 3]; 4] = [
            [words.nth(6).unwrap().parse().unwrap(), 0, 0],
            [words.nth(5).unwrap().parse().unwrap(), 0, 0],
            [words.nth(5).unwrap().parse().unwrap(), words.nth(2).unwrap().parse().unwrap(), 0],
            [words.nth(5).unwrap().parse().unwrap(), 0, words.nth(2).unwrap().parse().unwrap()],
        ];

        let mut result = 0;
        find_max(&costs, [0; 4], [1, 0, 0, 0], 0, 0, &mut result);
        result
    }).product::<u16>());
}
