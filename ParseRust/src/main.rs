use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let iterations = args[1].parse::<i64>().unwrap();

    let mut s: i64 = 0;
    for _x in 0..iterations {
        s = sum_line(args[2].to_string());
    }
    println!("{}", s);
}

fn sum_line(line: String) -> i64 {
    let fields_iterator = line.split('|').map(str::trim);
    let mut sum: i64 = 0;
    for text in fields_iterator {
        sum += text.parse::<i64>().unwrap();
    }
    return sum;
}
