extern crate colored;
use colored::*;

fn main() {
    let cards: [i32; 52] = [
        00, 01, 02, 03, 04, 05, 06, 07, 08, 09, 10, 11, 12, 
        13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 
        26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 
        39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51
    ];

    let mut stacks: [Vec<i32>; 7] = [
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new()
    ];
    let mut rest = Vec::new();

    let mut current = 0;
    let mut cards_vec = cards.to_vec();
    for x in cards.iter() {
        if current < 7 {
            for (i, item) in stacks.iter_mut().enumerate() {
                if i >= current {
                    item.push(cards_vec.pop().unwrap());
                }
            }
        } else {
            rest.push(x);
        }
        current += 1;
    }

    for (i, item) in stacks.iter().enumerate() {
        print!("{}]", i);
        for card in item.iter() {
            print!("{}", format(card));
        }
        println!("");
    }

    print!("S]");
    for card in rest.iter() {
        print!("{}", format(card));
    }
    
    println!("");
}

fn format(number: &i32) -> ColoredString {
    let face = number / 13;
    let mut card_string = match face {
        0 => String::from(" ♣"),
        1 => String::from(" ♦"),
        2 => String::from(" ♥"),
        3 => String::from(" ♠"),
        _ => String::from(" ?")
    };

    let card = number % 13;
    match card {
        // Match a single value
        0 =>  card_string.push_str("-AA "),
        10 => card_string.push_str("-BB "),
        11 => card_string.push_str("-DD "),
        12 => card_string.push_str("-KK "),
        _ => card_string.push_str(format!("-{:>02} ", card + 1).as_ref()),
    };

    if face % 2 == 0 {
        return card_string.red().on_white();
    }
    
    return card_string.black().on_white()
}
