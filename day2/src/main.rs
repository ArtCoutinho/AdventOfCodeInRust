// this is for part 2
fn is_repeated(num: u64) -> bool {
    let num_str = num.to_string();
    let mut number_of_chars: usize = 1;

    if num_str.len() % 2 == 1{
        for _ in 0..num_str.len()/2{
            let mut substring: String = String::new();
            substring.push_str(&num_str[0..number_of_chars]);
            substring = substring.repeat(num_str.len() / number_of_chars);
            if substring == num_str {
                dbg!(&substring);
                dbg!(&num_str);
                return true;
            }
            substring.clear();
            number_of_chars += 2;
        }
        return false;
    }
    
    for _ in 0..num_str.len()/2 {
        let mut substring: String = String::new();
        substring.push_str(&num_str[0..number_of_chars]);
        substring = substring.repeat(num_str.len() / number_of_chars);
        if substring == num_str {
            dbg!(&substring);
            dbg!(&num_str);
            return true; 
        }
        substring.clear();
        number_of_chars += 1;
    }
    false
}

// this is for part 1
#[allow(dead_code)]
fn is_repeated_twice(num: u64) -> bool {
    let num_str = num.to_string();
    
    if num_str.len() % 2 != 0 {
        return false;
    }
    
    let mid = num_str.len() / 2;
    let first_half = &num_str[0..mid];
    let second_half = &num_str[mid..];
    
    first_half == second_half
}

fn parse_input(input: &str) -> Vec<(u64, u64)> {
    let mut intervals: Vec<(u64, u64)> = Vec::new();
    let mut num: String = String::new();
    let mut num2: String = String::new();
    let mut first: bool = true;

    for char in input.chars(){
        if char.is_alphanumeric() && first{
            num.push_str(&char.to_string());
        }
        else if char == '-'{
            first = false;
        }
        else if char.is_alphanumeric(){
            num2.push_str(&char.to_string());
        }
        else{
            intervals.push((num.parse().unwrap(), num2.parse().unwrap()));
            num = String::new();
            num2 = String::new();
            first = true;
        }
    }
    intervals
}

fn add_repeated(intervals: Vec<(u64, u64)>) -> u64 {
    let mut sum: u64 = 0;
    for pair in intervals {
        for num in pair.0..=pair.1 {
            if is_repeated(num) {
                sum += num;
            }
        }
    }
    sum
}

fn main() {
    
    let input = std::fs::read_to_string("PATH TO INPUT.TXT HERE").expect("Failed to read input file");
    let intervals = parse_input(&input);
    println!("These are the intervals {:?}", intervals);
    let result = add_repeated(intervals);
    println!("Sum of repeated numbers: {}", result);
}

