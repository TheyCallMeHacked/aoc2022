use std::io::stdin;

fn main() {
    let mut line : String = String::new();
    let mut total : [i32; 2] = [0, 0];
    loop { 
        if stdin().read_line(&mut line).unwrap_or_default() == 0 {
            break;
        }
        line.pop();
        
        let round = line.split_whitespace().collect::<Vec<&str>>();

        for i in 0..=1 {
            total[i] += match *round.get(1).unwrap() {
                "X" => 1-(i as i32),
                "Y" => 2+(i as i32),
                "Z" => 3+3*(i as i32),
                 _  => 0,
            };

            total[i] += rps(&round, i as i32);
        }

        line.clear();
    }
    println!("--- Day 2: Rock Paper Scissors ---\n");
    println!("Puzzle 1: {}\n", total[0]);
    println!("Puzzle 2: {}", total[1]);
}

fn rps(round: &Vec<&str>, ex: i32) -> i32 {
    let you = round.get(1).unwrap();
    let opponent = round.get(0).unwrap();
    
    match (ex, *you, *opponent) {
        (0, "X", "A") |
        (0, "Y", "B") |
        (0, "Z", "C") => 3,

        (0, "X", "B") |
        (0, "Y", "C") |
        (0, "Z", "A") => 0,

        (0, "X", "C") |
        (0, "Y", "A") |
        (0, "Z", "B") => 6,

        (1, "X", "B") |
        (1, "Y", "A") |
        (1, "Z", "C") => 1,

        (1, "X", "C") |
        (1, "Y", "B") |
        (1, "Z", "A") => 2,

        (1, "X", "A") |
        (1, "Y", "C") |
        (1, "Z", "B") => 3,
        
        _ => 0,
    }
}
