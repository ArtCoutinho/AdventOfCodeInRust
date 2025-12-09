fn parse_input(input: &str) -> Vec<i32> {
    input
        .split_whitespace()
        .filter_map(|s| {
            if s.is_empty() {
                return None;
            }
            let (sign, num) = s.split_at(1);
            num.parse::<i32>().ok().map(|n| match sign {
                "L" => -n,
                "R" => n,
                _ => n, // default to positive if invalid prefix
            })
        })
        .collect()
}

fn solve(numbers: Vec<i32>) -> i32 {
    let mut current: i32 = 50;
    let mut count: i32 = 0;
    
    for num in numbers {
        for _ in 0..num.abs() {
            if num < 0 {
                current -= 1;
            } else {
                current += 1;
            }
            if current == 0 || current % 100 == 0 {
                count += 1;
            }
        }
    }
    count
}

fn main() {
    let input = std::fs::read_to_string("PATH TO INPUT.TXT HERE").expect("Failed to read input file");
    let numbers = parse_input(&input);
    let result = solve(numbers);
    println!("Result: {}", result);
}