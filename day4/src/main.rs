const LINE_SIZE: usize = 136;

fn parse_input(input: &str) -> String {
    input
        .lines()
        .collect::<Vec<&str>>()
        .join("")
        .to_string()
}

struct Neighbours {
    top_right: char,
    top_left: char,
    bottom_right: char,
    bottom_left: char,
    top: char, 
    bottom: char,
    left: char,
    right: char,
}
    
struct Roll {
    top_edge: bool,
    bottom_edge: bool,
    left_edge: bool,
    right_edge: bool,
    neighbours: Neighbours,
}

fn count_valid_rolls(parsed_input: String) -> (u64, Vec<usize>) {
    let mut valid: u64 = 0;
    let mut valid_positions: Vec<usize> = Vec::new();
    
    for (i, char) in parsed_input.chars().enumerate() {
        if char == '.' {
            continue;
        }

        let top_edge = i < LINE_SIZE;
        let bottom_edge = i > 18360;
        let left_edge = i % LINE_SIZE == 0;
        let right_edge = i % LINE_SIZE == LINE_SIZE - 1;
        
        
        let top_right = if !top_edge && !right_edge {
            parsed_input.as_bytes()[i - (LINE_SIZE - 1)] as char
        } else {
            '.' 
        };
        let top_left = if !top_edge && !left_edge {
            parsed_input.as_bytes()[i - (LINE_SIZE + 1)] as char
        } else {
            '.' 
        };
        let bottom_right = if !bottom_edge && !right_edge {
            parsed_input.as_bytes()[i + (LINE_SIZE + 1)] as char
        } else {
            '.' 
        };
        let bottom_left = if !bottom_edge && !left_edge {
            parsed_input.as_bytes()[i + (LINE_SIZE - 1)] as char
        } else {
            '.' 
        };
        let top = if !top_edge {
            parsed_input.as_bytes()[i - LINE_SIZE] as char
        } else {
            '.' 
        };
        let bottom = if !bottom_edge {
            parsed_input.as_bytes()[i + LINE_SIZE] as char
        } else {
            '.' 
        };
        let right = if !right_edge {
            parsed_input.as_bytes()[i + 1] as char
        } else {
            '.' 
        };
        let left = if !left_edge {
            parsed_input.as_bytes()[i - 1] as char
        } else {
            '.' 
        };
        
        let roll = Roll {
            top_edge,
            bottom_edge,
            left_edge,
            right_edge,
            neighbours: Neighbours {
                top_right,
                top_left,
                bottom_right,
                bottom_left,
                top,
                bottom,
                left,
                right,
            },
        };
        
        let neigh = &roll.neighbours;
        
        let neighbors = [
            neigh.top_right,
            neigh.top_left,
            neigh.bottom_right,
            neigh.bottom_left,
            neigh.top,
            neigh.bottom,
            neigh.left,
            neigh.right,
        ];
            
        let adj_count = neighbors.iter().filter(|&&c| c == '@').count();
        if adj_count < 4{
            valid_positions.push(i);
            valid += 1;
        }    
            
    }
    (valid, valid_positions)
}

fn loop_over_rolls(parsed_input: String) -> u64 {
    let mut no_more_valid: bool = false;
    let mut sum: u64 = 0;
    let mut current_input = parsed_input.clone();
    let mut valid_rolls_positions: Vec<usize> = Vec::new();

    while !no_more_valid {
        no_more_valid = true;
        let valid_rolls: (u64, Vec<usize>) = count_valid_rolls(current_input.clone());
        
        if valid_rolls.0 > 0 {
            no_more_valid = false;
            sum += valid_rolls.0;
            valid_rolls_positions = valid_rolls.1;
        }
        for pos in valid_rolls_positions.iter() {
            let mut input_bytes = current_input.into_bytes();
            input_bytes[*pos] = '.' as u8;
            current_input = String::from_utf8(input_bytes).expect("Invalid UTF-8");
        }
    }

    sum
}

fn main() {
    let input = std::fs::read_to_string("PATH TO INPUT.TXT HERE").expect("Failed to read input file");
    let parsed_input = parse_input(&input);
    let valid_rolls = loop_over_rolls(parsed_input);
    println!("{}", valid_rolls);
}
