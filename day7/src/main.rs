fn get_tree(input: &str) -> Vec<String> {
    input
        .lines()
        .map(|s| s.to_string())
        .filter(|s| !s.is_empty())
        .collect()
}

fn beam_down(tree: &Vec<String>) -> Vec<String> {
    let mut shiny_tree: Vec<String> = Vec::new(); 
    let mut count_splits = 0;
 
    for (i, line) in tree.iter().enumerate() {
        let mut current_line: String = String::new();
        //let mut index: usize = 0;
        let mut inner_index: usize = 0;
        let mut skip_one: bool = false;
        
        
        for char in line.chars(){
            if skip_one {
                skip_one = false;
                //inner_index += 1;
                continue;
            }
            

            if i == 0{
                println!("first line");
                current_line.push_str(line);
                break;
            }

            if char == '.' {
                if shiny_tree[i-1].chars().nth(inner_index) == Some('S'){
                    println!("S found");
                    current_line.push('|');                                      
                }
                if shiny_tree[i-1].chars().nth(inner_index) == Some('.') {
                    //println!(". found");
                    current_line.push('.');                   
                }
                if shiny_tree[i-1].chars().nth(inner_index) == Some('|') {
                    //println!("| found");
                    current_line.push('|');                                    
                }
                if shiny_tree[i-1].chars().nth(inner_index) == Some('^'){
                    //println!("^ found");
                    current_line.push('.');                    
                }               
                inner_index += 1;
            }

            if char == '^' {
                if shiny_tree[i-1].chars().nth(inner_index) == Some('S'){ //leaving this here, but wont happen
                    current_line.push('|');                    
                }
                if shiny_tree[i-1].chars().nth(inner_index) == Some('.') {
                    current_line.push('^');                   
                }
                if shiny_tree[i-1].chars().nth(inner_index) == Some('|'){
                    //println!("Split found at line {}, index {}", i, inner_index);
                    //println!("Current line before split: {}", current_line);
                
                    current_line.pop();
                    current_line.push('|');
                    current_line.push('^');
                    current_line.push('|');

                    //println!("Current line after split: {}", current_line);
                    skip_one = true;
                    inner_index += 1;
                    count_splits += 1;
                    
                }
                if shiny_tree[i-1].chars().nth(inner_index) == Some('^'){
                    current_line.push('.');                    
                }
                inner_index += 1;               
            }
        }
        
        //add line and clear it
        shiny_tree.push(current_line.clone());
        current_line.clear();
    }
    
    println!("Total splits: {}", count_splits);
    shiny_tree
}

fn make_number_tree(tree: &Vec<String>) -> Vec<String> {
    let mut number_tree: Vec<String> = Vec::new();

    for (i, line) in tree.iter().enumerate() {
        let mut current_line: String = String::new();
        let mut inner_index: usize = 0;
        let mut skip_one: bool = false;

        for char in line.chars(){
            if skip_one {
                skip_one = false;
                //inner_index += 1;
                continue;
            }
            
            if i == 0{
                println!("first line");
                current_line.push_str(line);
                break;
            }

            if char == '.' {
                if number_tree[i-1].chars().nth(inner_index) == Some('S'){
                    println!("S found");
                    current_line.push('1');                                      
                }
                if number_tree[i-1].chars().nth(inner_index) == Some('.') {
                    //println!(". found");
                    //push whitespace
                    current_line.push('.');                   
                }
                if let Some(ch) = number_tree[i-1].chars().nth(inner_index) {
                    if ch.is_numeric() {
                        //println!("{} found, at {}", ch, inner_index);
                        current_line.push(ch);                                    
                    }
                }
                if number_tree[i-1].chars().nth(inner_index) == Some('^'){
                    //println!("^ found");
                    current_line.push('.');                    
                }               
                inner_index += 1;
            }

            if char == '^' {
                if number_tree[i-1].chars().nth(inner_index) == Some('.') {
                    current_line.push('^');                   
                }

                if let Some(ch) = number_tree[i-1].chars().nth(inner_index) {
                    if ch.is_numeric() {   
                        //println!("Split found at line {}, index {}", i, inner_index);
                        //println!("Current line before split: {}", current_line);

                    
                        current_line.pop();
                        current_line.push(ch);
                        current_line.push('^');
                        current_line.push(ch);

                        //println!("Current line after split: {}", current_line);
                        skip_one = true;
                        inner_index += 1;
                    }
                    
                }

                if number_tree[i-1].chars().nth(inner_index) == Some('^'){
                    current_line.push('^');                    
                }

                inner_index += 1;               
            }
        }

        number_tree.push(current_line.clone());
        current_line.clear();
    }


    number_tree  
}

fn add_up_tree(number_tree: &Vec<String>) -> u64 {
    let mut values_per_line: Vec<Vec<u64>> = vec![];
    let mut tree_ends: Vec<u64> = vec![0; 141];
    println!("{:?}", tree_ends);
    
    for (i, line) in number_tree.iter().enumerate() {
        values_per_line.push(Vec::new());
        for char in line.chars(){
            if char.is_numeric(){
                let digit = char.to_digit(10).unwrap() as u64;
                if values_per_line.len() <= i {
                    values_per_line.push(vec![digit]);
                } else {
                    values_per_line[i].push(digit);
                }
            }
            else{
                values_per_line[i].push(0);
            }                
            
        }
    }
    
    
    
    for line in &values_per_line {
        println!("{:?}", line);
    }
    
    println!("\nAdding up values:\n");
    tree_ends[70] = 1;
    for i in 0..values_per_line.len() {
        for j in 0..values_per_line[i].len() {
            if values_per_line[i][j] > 0 && i < 141 {
                if values_per_line[i + 1][j] == 0 && i > 0 && i < tree_ends.len() - 1 {
                    tree_ends[j - 1] += tree_ends[j];
                    tree_ends[j + 1] += tree_ends[j];
                    tree_ends[j] = 0;
                }
            }
        }
        if i % 2 == 0 {
            println!("{:?} -> line {}", tree_ends, i);
        }
    }

    let mut total: u64 = 0;
    for val in tree_ends {
        total += val;
    }
    
    total

}


fn main() {
    let input = std::fs::read_to_string("/home/aqc/rustProjects/adventofcode/day7/input7.txt").expect("Failed to read input file");
    let tree = get_tree(&input);
    
    println!("Original tree: \n");
    for line in &tree {
        println!("{}", line);
    }
    
    println!("\nShiny tree: \n");
    let shiny_tree = beam_down(&tree);
    for line in &shiny_tree {
        println!("{}", line);
    }
    
    println!("\nMaking number tree: \n");
    let mut number_tree = make_number_tree(&tree);
    for line in &number_tree {
        println!("{}", line);
    }
    
    println!("\nAdding up tree: \n");
    let total = add_up_tree(&number_tree);
    println!("Total ways to reach the bottom: {}", total);
    
    
    
}
