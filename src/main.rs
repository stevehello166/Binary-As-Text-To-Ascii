use std::io;

fn main() {
    loop{
        println!("Turn ones and zeros text into bytes!");
        println!("Input your string");
    
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("failed to read line");

        let mut parsed_vec: Vec<String> = Vec::new();

        let mut charvec: Vec<char> = Vec::new();

        for character in input.chars() {
            if character == '0' || character == '1' {
                charvec.push(character)
            }
            if charvec.len() == 8 {
                let collection_string: String = charvec.iter().collect();
                parsed_vec.push(collection_string);
                charvec = Vec::new()
            }
        }
        println!("{:?}", parsed_vec);

        let mut byte_vec: Vec<u8> = Vec::new();

        for unfinished_byte in parsed_vec{
            let mut iterations = 1;
            let mut byte_parts: Vec<u8> = Vec::new();
            for ch in unfinished_byte.chars() {
                if ch == '1'{
                    match iterations {
                        1 => byte_parts.push(128), 
                        2=> byte_parts.push(64), 
                        3=> byte_parts.push(32),
                        4=> byte_parts.push(16), 
                        5=> byte_parts.push(8), 
                        6 => byte_parts.push(4), 
                        7 => byte_parts.push(2), 
                        8 => byte_parts.push(1),
                        _ => {panic!("TOO MANY")}
                    }
                }
                
                iterations += 1;
            }
            byte_vec.push(byte_parts.iter().sum());
        }

        let mut charvec: Vec<char> = Vec::new();
        for ch in byte_vec.iter(){
            charvec.push(*ch as char);
            
            print!("{}", *ch as char)
        }
        println!();
    }
}
