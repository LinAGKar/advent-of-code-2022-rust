use std::io::Read;

fn get_val(
    monkeys: &mut Vec<(Option<i64>, Option<(usize, char, usize)>)>,
    id: usize,
) -> i64 {
    if let Some(val) = monkeys[id].0 {
        val
    } else {
        let (a_id, op, b_id) = monkeys[id].1.unwrap();
        let a = get_val(monkeys, a_id);
        let b = get_val(monkeys, b_id);
        let val = match op {
            '+' => a + b,
            '-' => a - b,
            '*' => a * b,
            '/' => a / b,
            _ => panic!(),
        };
        monkeys[id].0 = Some(val);
        val
    }
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut mapping= std::collections::HashMap::new();

    let mut get_id = |name: &str| {
        if let Some(&id) = mapping.get(name) {
            id
        } else {
            let id = mapping.len();
            mapping.insert(name.to_string(), id);
            id
        }
    };

    let mut monkeys = Vec::new();

    for line in input.lines() {
        let mut words = line.split_whitespace();
        let id = get_id(words.next().unwrap().trim_matches(':'));
        let a = words.next().unwrap();
        if monkeys.len() <= id {
            monkeys.resize(id + 1, (None, None));
        }
        if let Ok(num) = a.parse::<i64>() {
            monkeys[id].0 = Some(num);
        } else {
            let op = words.next().unwrap();
            let b = words.next().unwrap();
            monkeys[id].1 = Some((
                get_id(a),
                op.chars().next().unwrap(),
                get_id(b),
            ));
        }
    }

    println!("{}", get_val(&mut monkeys, mapping["root"]));
}
