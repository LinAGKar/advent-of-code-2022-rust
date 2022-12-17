use std::collections::{VecDeque, HashMap, BTreeSet};
use std::io::Read;

fn parse_valve_id(id: &str) -> usize {
    let mut letters = id.chars().map(|c| c as usize - 'A' as usize);
    letters.next().unwrap() + letters.next().unwrap() * 26
}

fn find_paths(
    valves: &[(u16, Vec<(u16, usize)>)],
    visited: u64,
    cost: u16,
    released: u16,
    pos: usize,
    best_by_visited: &mut HashMap<u64, u16>,
    possibilities: &mut BTreeSet<(u16, u64)>,
) {
    const MAX_TIME: u16 = 26;

    let best_this_visited = best_by_visited.entry(visited).or_default();
    if released > *best_this_visited {
        possibilities.remove(&(*best_this_visited, visited));
        *best_this_visited = released;
        possibilities.insert((released, visited));
    }

    for &(this_cost, new_pos) in &valves[pos].1 {
        let new_cost = cost + this_cost;

        if visited & (1 << new_pos) != 0 || new_cost > MAX_TIME {
            continue;
        }
        let new_visited = visited | (1 << new_pos);
        let new_released = released + (MAX_TIME - new_cost) * valves[new_pos].0;

        find_paths(valves, new_visited, new_cost, new_released, new_pos, best_by_visited, possibilities);
    }
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut valves = vec![(0u16, vec![]); 26 * 26];
    let mut working_valves = HashMap::new();

    for line in input.lines() {
        let mut words = line.split_whitespace();
        let valve = parse_valve_id(words.nth(1).unwrap());
        let rate = words.nth(2).unwrap().trim_start_matches("rate=").trim_end_matches(';').parse().unwrap();
        let tunnels = words.skip(4).map(|w| parse_valve_id(w.trim_end_matches(','))).collect();
        valves[valve] = (rate, tunnels);
        if rate > 0 || valve == 0 {
            working_valves.insert(valve, working_valves.len());
        }
    }

    let mut queue = VecDeque::new();
    let mut valve_graph = vec![(0u16, vec![]); working_valves.len()];

    for (&id, &index) in &working_valves {
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
                    tunnels.push((new_cost + 1, working_valves[&new_pos]));
                    working_visited += 1;
                    if working_visited == working_valves.len() {
                        break 'outer;
                    }
                }
                queue.push_back((new_cost, new_pos));
            }
        }

        assert!(working_visited == working_valves.len());
        valve_graph[index] = (valves[id].0, tunnels);
    }

    let start = working_valves[&0];
    let mut possibilities = BTreeSet::new();
    find_paths(&valve_graph, 1 << start, 0, 0, start, &mut HashMap::new(), &mut possibilities);

    let mut best = 0;

    for &(released_a, visited_a) in possibilities.iter().rev() {
        for &(released_b, visited_b) in possibilities.iter().rev() {
            if released_a + released_b <= best {
                break;
            }
            if (visited_a & visited_b) != 1 << start {
                continue;
            }
            best = released_a + released_b;
        }
    }

    println!("{}", best);
}
