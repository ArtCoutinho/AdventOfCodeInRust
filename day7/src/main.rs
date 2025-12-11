fn get_tree(input: &str) -> Vec<String> {
    input
        .lines()
        .map(|s| s.to_string())
        .filter(|s| !s.is_empty())
        .collect()
}

fn beam_down(tree: &Vec<String>) -> Vec<String> {
    let mut shiny_tree: Vec<String> = Vec::new();
    let line_size = tree[0].len();
    //let mut beam_coords: Vec<usize> = Vec::new();
    let mut count_splits = 0;
    
    for (i, line) in tree.iter().enumerate() {
        let mut current_line: String = String::new();
        //let mut index: usize = 0;
        let mut inner_index: usize = 0;
        let mut skip_one: bool = false;
        let mut skip_two: bool = false;
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
                    skip_two = true;
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
    println!("Number of splits: {}", count_splits);
    shiny_tree
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
    //println!("this is the shiny tree so far {:?}", shiny_tree);
    //println!("Parsed tree: {:?}", tree);
}
