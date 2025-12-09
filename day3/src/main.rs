fn parse_input(input: &str) -> Vec<String>{
    input  
        .split('\n')
        .map(|s| s.to_string()).filter(|s| !s.is_empty())
        .collect()
}

fn find_highest(bank_input: String) -> u64 {
    let mut current_highest: u64 = 0;
    let mut index = 0;
    let mut remaining = 12;

    for (i, ch) in bank_input.chars().enumerate() {
        if let Some(digit) = ch.to_digit(10) {
            if digit as u64 > current_highest && i <= bank_input.len() - remaining {
                current_highest = digit as u64 ;
                index = i+1;
            }
        }
    }

    let mut highest: u64 = current_highest * 10;
    current_highest = 0;
    remaining -= 1;
    
    while remaining > 0 {
        let sliced_input = &bank_input[index..];
        let mut temp = 0;
        for (i, ch) in sliced_input.chars().enumerate() {
            if let Some(digit) = ch.to_digit(10) {
                if digit as u64 > current_highest && i <= sliced_input.len() - remaining {
                    current_highest = digit as u64 ;
                    temp = i;
                }
            }
        }
        
        index += temp+1;
        highest += current_highest;
        if highest >= 100000000000{
            return highest;
        }
        if index > 100 {
            return highest;
        }
        highest *= 10;
        current_highest = 0;
        remaining -= 1;

    }


    highest
    
}

fn add_highest(parsed_input: Vec<String>) -> u64 {
    let mut sum: u64 = 0;

    for num in parsed_input{
        let current = find_highest(num);
        sum += current;
        println!("{} {}",current , sum);
    }

    sum

}

fn main() {
    let input = std::fs::read_to_string("PATH TO INPUT.TXT HERE").expect("Failed to read input file");
    let parsed_input = parse_input(&input);

    let result = add_highest(parsed_input);

    println!("{}", result);
}

