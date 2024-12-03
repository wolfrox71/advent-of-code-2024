use std::fs::read_to_string;

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}

fn valid(line: String, char_to_skip: u32) -> bool {
    let mut desc : u8 = u8::MAX;
    let mut prev_val = i32::MAX;
    let mut current_char = 0;
    if line == " " {
        return false;
    }

    println!("Line: {}", line);
    for c_str in line.split(" ") {
        if c_str == " " {
             continue;
        } 
        let c = c_str.parse::<i32>().unwrap();

        current_char += 1;
        if current_char == char_to_skip { continue; }

        if prev_val != i32::MAX {
            println!("{}", c);
            println!("{}", prev_val);
            let diff = c-prev_val;

            if diff == 0 { return false; }

            if diff.abs() > 3 {
                println!("Diff too big, {}, invalid", diff);
                return false;
            }

            if desc == u8::MAX {
                desc = if diff > 0 { 0 } else { 1 } ;// 1 if desc, 0 if not, u8::MAX if not enough info yet
            }

            let current_desc = if diff > 0 { 0 } else {1};

            if current_desc != desc {
                println!("Not valid {} {} {} {} {}", prev_val, c, diff, desc, current_desc);
                return false;
            }
            

            println!("Diff between {} and {} = {}", prev_val, c, diff);
        }
        prev_val = c;
    }
    println!("Valid");
    true
}

fn main() {
    let filename = "input.txt";

    let return_val = read_lines(filename);
   
    let mut total = 0;
    let mut num_lines = 0;
    for line in return_val {
        if valid(line.clone(), u32::MAX) {
            total += 1;
        }
        
        else {
            // go through removing each char once to see if that fixes it
            let i : u32 = 0;
            for i in 0..=line.clone().split(" ").count() {

               println!("Skipping char {}", i);

               if valid(line.clone(), i.try_into().unwrap()) {
                   total += 1;
                   break;
               }
            }
        }
        num_lines += 1;
        println!("******************");
    }

    println!("Total: {} out of {}", total, num_lines);
}
