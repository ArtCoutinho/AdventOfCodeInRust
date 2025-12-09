fn parse_input(input: &str) -> (Vec<(u64, u64)>, Vec<u64>) {
    let mut intervals: Vec<(u64, u64)> = Vec::new();
    let mut numbers: Vec<u64> = Vec::new();
    let mut intervals_string: String = String::new();
    let mut ids_string: String = String::new();
    let mut done_with_intervals: bool = false;
  
    
    for (i, char) in input.chars().enumerate() {
        if char == '\n' && input.as_bytes()[i-1] as char == '\n' {
            done_with_intervals = true;
        }
        if !done_with_intervals{
            intervals_string.push(char);
        }
        else{
            ids_string.push(char);
        }
    } 

    intervals_string
        .lines()
        .for_each(|line| {
            let parts: Vec<&str> = line.split('-').collect();
            if parts.len() == 2 {
                let start = parts[0].trim().parse::<u64>().unwrap();
                let end = parts[1].trim().parse::<u64>().unwrap();
                intervals.push((start, end));
            }
        });
    
    ids_string
        .lines()
        .for_each(|line| {
            if line.trim().is_empty() {
                return;
            }
            let id = line.trim().parse::<u64>().unwrap();
            numbers.push(id);
        });
    

    
    (intervals, numbers)
}

fn find_valid_numbers(intervals: &Vec<(u64, u64)>, numbers: &Vec<u64>) -> u64{
    let mut valid_numbers: Vec<u64> = Vec::new();
    
    for num in numbers.iter(){
        for (start, end) in intervals.iter(){
            if num >= start && num <= end{
                valid_numbers.push(*num);
                break;
            }
        }
    }

    println!("These are the valid Ids {:?}", valid_numbers);
    return valid_numbers.len() as u64;
}

fn find_all_valid_numbers(intervals: &Vec<(u64, u64)>) -> u64{
    
    let mut non_duplicate_intervals: Vec<(u64, u64)> = reduce_intervals(intervals);
    let mut reduced: bool = false;
    while !reduced{
        if non_duplicate_intervals == reduce_intervals(&non_duplicate_intervals){
            println!("No overlapping intervals found, proceeding...");
            reduced = true;
        }
        else{
            non_duplicate_intervals = reduce_intervals(&non_duplicate_intervals);
        }
    }
    
    println!("These are the non duplicate intervals {:?}, length {} \n", non_duplicate_intervals, non_duplicate_intervals.len());
    
    let mut total_valid: u64 = 0;
    
   
    for (start, end) in non_duplicate_intervals.iter(){

        total_valid += end + 1; // doing like this to avoid overflow
        total_valid -= start;
    }

    total_valid
}

fn reduce_intervals(intervals: &Vec<(u64, u64)>) -> Vec<(u64, u64)>{
    let mut non_duplicate_start: Vec<u64> = Vec::new();
    let mut non_duplicate_end: Vec<u64> = Vec::new();
    
    for (start, end) in intervals.iter(){
        let mut current_start: u64 = *start;
        let mut current_end: u64 = *end;
        for (start2, end2) in intervals.iter(){
            if start2 == start && end2 == end{ //equal
                continue;
            }
            else if start2 >= start && end2 <= end{ //interval1 includes interval2
                continue;
            }
            else if start2 <= start && end2 >= end{ //interval2 includes interval1
                current_start = *start2;
                current_end = *end2;
            }
            else if start2 <= start && end2 >= start{ //interval2 starts before interval1, ends inside
                current_start = *start2;
            }
            else if start2 <= end && end2 >= end{ //interval2 starts inside interval1, ends after
                current_end = *end2;
            }
        }
        non_duplicate_start.push(current_start);
        non_duplicate_end.push(current_end);
    }

    let mut non_duplicate_intervals: Vec<(u64, u64)> = Vec::new();
    for i in 0..non_duplicate_start.len(){
        non_duplicate_intervals.push((non_duplicate_start[i], non_duplicate_end[i]));
    }
    non_duplicate_intervals.sort_unstable();
    non_duplicate_intervals.dedup();
    non_duplicate_intervals
}


fn main() {
    let input = std::fs::read_to_string("PATH TO INPUT.TXT HERE").expect("Failed to read input file");
    let (intervals, numbers) = parse_input(&input);
    //let result = find_valid_numbers(&intervals, &numbers);
    //println!("Parsed Intervals: {:?}", intervals);
    //println!();
    let all_valid = find_all_valid_numbers(&intervals);

    //println!("There are {} valid Ids", result);
    println!("There are {} valid Ids in total", all_valid);
    //println!("Parsed Numbers: {:?}", numbers);
    //println!("There are {} valid Ids", result)

}
