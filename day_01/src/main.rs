use std::io::stdin;
use std::cmp::max;

fn main() {
    let mut line : String = String::new();
    let mut max1 : i32 = 0;
    let mut max2 : i32 = 0;
    let mut max3 : i32 = 0;
    let mut curr : i32 = 0;
    loop { 
        let b = stdin().read_line(&mut line).unwrap_or_default();
        line.pop();
        match b {
            0 => {
                new_max(&mut curr, &mut max1, &mut max2, &mut max3);
                break;
            },
            1 => new_max(&mut curr, &mut max1, &mut max2, &mut max3),
            _ => curr += line.parse::<i32>().unwrap(),
        }
        line.clear();
    }
    println!("--- Day 1: Calorie Counting ---\n");
    println!("Puzzle 1: {}\n", max1);
    println!("Puzzle 2: {}\n", max1 + max2 + max3);
}

fn new_max(val: &mut i32, max1: &mut i32, max2: &mut i32, max3: &mut i32) {
    let tmp : i32 = *val + *max1;
    *max1 = max(*val, *max1);
    *val = tmp - *max1;
    let tmp : i32 = *val + *max2;
    *max2 = max(*val, *max2);
    *val = tmp - *max2;
    let tmp : i32 = *val + *max3;
    *max3 = max(*val, *max3);
    *val = tmp - *max3;

    *val = 0;
}
