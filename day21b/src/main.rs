use std::io::Read;

#[derive(Debug)]
enum Result {
    Val(i64),
    Humn(Vec<(i64, Op)>),
}

#[derive(Debug, Clone, Copy)]
enum Op {
    Add,
    Sub,
    Lsub,
    Mult,
    Div,
    Ldiv,
}

fn get_val(
    monkeys: &mut Vec<(Option<i64>, Option<(usize, Op, usize)>)>,
    id: usize,
    humn_id: usize,
) -> Result {
    if id == humn_id {
        Result::Humn(Vec::new())
    } else if let Some(val) = monkeys[id].0 {
        Result::Val(val)
    } else {
        let (a_id, op, b_id) = monkeys[id].1.unwrap();
        match (get_val(monkeys, a_id, humn_id), get_val(monkeys, b_id, humn_id)) {
            (Result::Val(a), Result::Val(b)) => {
                let val = match op {
                    Op::Add => a + b,
                    Op::Sub => a - b,
                    Op::Mult => a * b,
                    Op::Div => a / b,
                    _ => panic!(),
                };
                monkeys[id].0 = Some(val);
                Result::Val(val)
            }

            (Result::Val(val), Result::Humn(mut humn)) => {
                humn.push((val, match op {
                    Op::Sub => Op::Lsub,
                    Op::Div => Op::Ldiv,
                    x => x,
                }));
                Result::Humn(humn)
            }

            (Result::Humn(mut humn), Result::Val(val)) => {
                humn.push((val, op));
                Result::Humn(humn)
            }

            _ => panic!(),
        }
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
                match op.chars().next().unwrap() {
                    '+' => Op::Add,
                    '-' => Op::Sub,
                    '*' => Op::Mult,
                    '/' => Op::Div,
                    _ => panic!(),
                },
                get_id(b),
            ));
        }
    }

    let result = get_val(&mut monkeys, mapping["root"], mapping["humn"]);

    if let Result::Humn(result) = result {
        println!("{}", result.into_iter().rev().reduce(|(a, _), (b, op)| {
            (match op {
                Op::Add => a - b,
                Op::Sub => a + b,
                Op::Lsub => b - a,
                Op::Mult => a / b,
                Op::Div => a * b,
                Op::Ldiv => b / a,
            }, Op::Add)
        }).unwrap().0);
    }
}
