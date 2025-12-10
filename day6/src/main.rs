fn parse_input(input: &str) -> Vec<u64> {
    let mut numbers: Vec<u64> = Vec::new();
    let lines: Vec<&str> = input.lines().collect();
    let mut rows_of_numbers: Vec<Vec<u64>> = vec![Vec::new(); 4];
    let mut row_of_operations: Vec<String> = Vec::new();

    //println!("These are the lines {:?}", lines);
    
    let mut index: usize = 0;
    for line in lines.iter(){
        let mut digit: String = String::new();
        let mut whitespace_found: bool = true;
        for char in line.chars(){
            if char.is_numeric() && !whitespace_found{
                digit.push(char);
                whitespace_found = true;
                continue;
            }
            if char == ' ' {
                if !digit.is_empty(){
                    let num = digit.parse::<u64>().unwrap();
                    rows_of_numbers[index].push(num);
                    digit.clear();
                }
                whitespace_found = true;
                continue;
            }
            if char.is_numeric() && whitespace_found{
                digit.push(char);
                whitespace_found = false;
                continue;
            } 
        }
        if !digit.is_empty(){
            let num = digit.parse::<u64>().unwrap();
            rows_of_numbers[index].push(num);
            digit.clear();
        }
        if index >= 3 {
            break;
        }
        index += 1;
    }


    for line in lines.iter().skip(4){
        for char in line.chars(){
            if char != ' ' {
                row_of_operations.push(char.to_string());
            }
        }
    }
    
    for i in 0..row_of_operations.len(){
        if row_of_operations[i] == "+"{
            let sum: u64 = rows_of_numbers[0][i] + rows_of_numbers[1][i] + rows_of_numbers[2][i] + rows_of_numbers[3][i];
            numbers.push(sum);
        }
        if row_of_operations[i] == "*"{
            let product: u64 = rows_of_numbers[0][i] * rows_of_numbers[1][i] * rows_of_numbers[2][i] * rows_of_numbers[3][i];
            numbers.push(product);
        }
    }

    //println!("These are the rows of numbers {:?}", rows_of_numbers);
    //println!("These are the row of operations {:?}", row_of_operations);
    numbers   
}

fn parse_input_part2(input: &str) -> Vec<u64> {
    let mut numbers: Vec<u64> = Vec::new();
    let lines: Vec<&str> = input.lines().collect();
    let line_size = input.lines().next().unwrap().len();
    let mut rows_of_numbers: Vec<String> = Vec::new();
    let mut row_of_operations: Vec<String> = Vec::new();

    for (i, char) in lines[0].chars().enumerate(){
        let mut digit1 = String::from("");
        
        if char.is_numeric(){
            digit1.push(char);
            for j in 1..4{
                let index = i + ((line_size + 1) * j);
                if let Some(next_char) = input.chars().nth(index){
                    //println!("The next char is {} from index {}", next_char, index);
                    if next_char.is_numeric(){
                        digit1.push(next_char);
                    }
                }                
            }
        }

        if char == ' ' {
            let mut empty: bool = true;
            for j in 1..4{
                let index = i + ((line_size + 1) * j);
                if let Some(next_char) = input.chars().nth(index){
                    //println!("The next char is {} from index {}", next_char, index);
                    if next_char.is_numeric(){
                        digit1.push(next_char);
                        empty = false;
                    }
                }                
            }
            if empty{
                rows_of_numbers.push("0".to_string());
            }
        }

        if !digit1.is_empty(){
            rows_of_numbers.push(digit1);
        }
    }

    for line in lines.iter().skip(4){
        for char in line.chars(){
            if char != ' ' {
                row_of_operations.push(char.to_string());
            }
        }
    }

    let new_line_size = rows_of_numbers.len() / 4;
    
    
    //the problemn is here, when 
    let mut index1 = 0;
    for i in 0..row_of_operations.len(){
        if row_of_operations[i] == "+"{
            let mut sum: u64 = 0;
            while rows_of_numbers[index1] != "0" && index1 < rows_of_numbers.len(){
                println!("Adding number at index1 {}: {}", index1, rows_of_numbers[index1]);
                let num = rows_of_numbers[index1].parse::<u64>().unwrap();
                sum += num;
                if index1 == rows_of_numbers.len() - 1{
                    break;
                }
                index1 += 1;
            }
            if index1 < rows_of_numbers.len() {
                index1 += 1;   
            }
            
            numbers.push(sum);
            
        }

        else if row_of_operations[i] == "*"{
            let mut product: u64 = 1;
            while rows_of_numbers[index1] != "0" && index1 < rows_of_numbers.len(){
                let num = rows_of_numbers[index1].parse::<u64>().unwrap();
                product *= num;
                if index1 == rows_of_numbers.len() - 1{
                    break;
                }
                index1 += 1;
            }
            if index1 < rows_of_numbers.len() {
                index1 += 1;   
            }
            numbers.push(product);
        }
    }

    println!("These are the row of operations {:?}, length {}", row_of_operations, row_of_operations.len());
    println!("These are the rows of numbers {:?}, length {}", rows_of_numbers, rows_of_numbers.len());
    
    numbers
}

fn add_numbers(numbers: &Vec<u64>) -> u64{
    let mut sum: u64 = 0;
    for num in numbers.iter(){
        sum += *num;
    }
    sum
}



fn main() {
    let input = std::fs::read_to_string("/home/aqc/rustProjects/adventofcode/day6/input6.txt").expect("Failed to read input file");
    //let numbers = parse_input(&input);
    let numbers_part2 = parse_input_part2(&input);
    //let sum = add_numbers(&numbers);
    //println!("The sum of the numbers is: {} \n", sum);
    println!("These are the numbers {:?}, length {}\n", numbers_part2, numbers_part2.len());
    let sum_part2 = add_numbers(&numbers_part2);
    println!("The sum of the numbers for part 2 is: {}\n", sum_part2);

}
