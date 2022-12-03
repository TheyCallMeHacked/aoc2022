use std::io::stdin;

fn main() {
    let mut group : [String; 3] = [String::new(), String::new(), String::new()];
    let mut total : [i32;2] = [0, 0];
    loop {
        let mut exit = false;
        for mut line in &mut group {
            if stdin().read_line(&mut line).unwrap_or_default() == 0 {
                exit = true;
                break;
            }
            line.pop();

            let mid = line.len()/2;
            let half = [&line[0..mid], &line[mid..line.len()]];
            let mut found = false;

            for c in half[0].chars() {
                if !found {
                    total[0] += match half[1].find(c) {
                        Some(_) => {found = true; priority(c)},
                        None    => 0,
                    };
                }
            }
        }
        if exit {break;}

        total[1] += priority(intersection(&group));
        
        for line in &mut group {
            line.clear();
        }
    }

    println!("--- Day 2: Rock Paper Scissors ---\n");
    println!("Puzzle 1: {}\n", total[0]);
    println!("Puzzle 2: {}", total[1]);
}

fn priority (c: char) -> i32 {
    if c.is_lowercase() {
        (c as i32) - ('a' as i32) + 1
    }
    else if c.is_uppercase() {
        (c as i32) - ('A' as i32) + 27
    }
    else {0}
}

fn intersection (g: &[String; 3]) -> char {
    let mut temp = Vec::new();
    for c in g[0].chars() {
        match g[1].find(c) {
            Some(_) => temp.push(c),
            None => (),
        }
    }

    let mut out = Vec::new();
    for c in temp {
        match g[2].find(c) {
            Some(_) => out.push(c),
            None => (),
        }
    }

    out.pop().unwrap()
}
