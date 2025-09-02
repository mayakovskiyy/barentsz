use rand::prelude::*;

fn main() {
    
    // needs to optimize ↓↓↓
    let letters = ['A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h', 'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p', 'Q', 'q', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'X', 'x', 'Y', 'y', 'Z', 'z', '!', '?', '/', '@', '#', '$', '%', ':', '^', '&', '*', '(', ')', '_', '-', '+', '=', '~', '`'];
    let mut char1: u8 = rand::random();
    let mut char2: u8 = rand::random();
    let mut char3: u8 = rand::random();
    let mut char4: u8 = rand::random();
    let mut char5: u8 = rand::random();
    let mut char6: u8 = rand::random();
    let mut char7: u8 = rand::random();
    let mut char8: u8 = rand::random();
    let mut char9: u8 = rand::random();
    let mut num2: u8 = rand::random();
    let mut num3: u8 = rand::random();
    let mut num4: u8 = rand::random();
    
    loop{
        if char1 > 70 || char2 > 70 || char3 > 70 || char4 > 70 || char5 > 70 || char6 > 70 || char7 > 70 || char8 > 70 || char9 > 70 {
        // needs to optimize ↓↓↓
            char1 = rand::random();
            char2 = rand::random();
            char3 = rand::random();
            char4 = rand::random();
            char5 = rand::random();
            char6 = rand::random();
            char7 = rand::random();
            char8 = rand::random();
            char9 = rand::random();
            num2 = rand::random();
            num3 = rand::random();
            num4 = rand::random();
        }
        else {
            println!("{}{}{}{}{}{}{}{}{}{}{}{}",
                letters[char1 as usize],
                num2, letters[char2 as usize],
                letters[char3 as usize],
                letters[char4 as usize],
                letters[char5 as usize],
                letters[char6 as usize],
                letters[char7 as usize],
                num3,
                letters[char8 as usize],
                letters[char9 as usize],
                num4,
            );
            break;
        }
    }

}
