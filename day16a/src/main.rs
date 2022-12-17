use std::collections::VecDeque;
use std::io::Read;

fn parse_valve_id(id: &str) -> usize {
    let mut letters = id.chars().map(|c| c as usize - 'A' as usize);
    letters.next().unwrap() + letters.next().unwrap() * 26
}

fn find_max_release(
    valves: &[(u16, Vec<(u16, usize)>)],
    visited: &mut [bool],
    cost: u16,
    pos: usize,
) -> u16 {
    const MAX_TIME: u16 = 30;

    let result = valves[pos].1.iter().filter_map(|&(this_cost, new_pos)| {
        let new_cost = cost + this_cost;
        if visited[new_pos] || new_cost > MAX_TIME {
            None
        } else {
            visited[new_pos] = true;
            let result = find_max_release(valves, visited, new_cost, new_pos);
            visited[new_pos] = false;
            Some(result + (MAX_TIME - new_cost) * valves[new_pos].0)
        }
    }).max().unwrap_or(0);
    result
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut valves = vec![(0u16, vec![]); 26 * 26];
    let working_valves: Vec<_> = input.lines().filter_map(|line| {
        let mut words = line.split_whitespace();
        let valve = parse_valve_id(words.nth(1).unwrap());
        let rate = words.nth(2).unwrap().trim_start_matches("rate=").trim_end_matches(';').parse().unwrap();
        let tunnels = words.skip(4).map(|w| parse_valve_id(w.trim_end_matches(','))).collect();
        valves[valve] = (rate, tunnels);
        if rate > 0 || valve == 0 {
            Some(valve)
        } else {
            None
        }
    }).collect();

    let mut queue = VecDeque::new();
    let mut valve_graph = vec![(0u16, vec![]); 26 * 26];

    for &id in &working_valves {
        let mut visited = [false; 26 * 26];
        visited[id] = true;
        queue.clear();
        queue.push_back((0, id));
        let mut tunnels = Vec::with_capacity(working_valves.len() - 1);
        let mut working_visited = 1;

        'outer: while let Some((cost, pos)) = queue.pop_front() {
            let new_cost = cost + 1;
            for &new_pos in &valves[pos].1 {
                if visited[new_pos] {
                    continue;
                }
                visited[new_pos] = true;
                if valves[new_pos].0 > 0 || new_pos == 0 {
                    tunnels.push((new_cost + 1, new_pos));
                    working_visited += 1;
                    if working_visited == working_valves.len() {
                        break 'outer;
                    }
                }
                queue.push_back((new_cost, new_pos));
            }
        }

        assert!(working_visited == working_valves.len());
        valve_graph[id] = (valves[id].0, tunnels);
    }

    let mut visited = vec![false; 26 * 26];
    visited[0] = true;
    let result = find_max_release(&valve_graph, &mut visited, 0, 0);

    println!("{}", result);
}
