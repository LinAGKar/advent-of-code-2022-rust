use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut lines = input.lines().peekable();
    let mut size_stack = Vec::new();
    let mut sum = 0;

    assert!(lines.next().unwrap() == "$ cd /");
    size_stack.push(0);
    while let Some(line) = lines.next().or_else(|| if size_stack.len() > 1 {
        Some("$ cd ..")
    } else {
        None
    }) {
        if line == "$ ls" {
            while let Some(&line2) = lines.peek().filter(|&&x| !x.starts_with("$")) {
                lines.next();
                if !line2.starts_with("dir") {
                    *size_stack.last_mut().unwrap() += line2.split_whitespace().next().unwrap().parse::<u32>().unwrap();
                }
            }
        } else if !line.starts_with("$ cd ") {
            panic!("Line {} was not cd or ls", line);
        } else if &line[5..] == ".." {
            let size = size_stack.pop().unwrap();
            if size <= 100000 {
                sum += size;
            }
            *size_stack.last_mut().unwrap() += size;
        } else {
            size_stack.push(0);
        }
    }

    println!("{}", sum);
}
