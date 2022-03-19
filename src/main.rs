use compile_time_run::run_command_str;
const WORD_LIST  : &'static str = run_command_str!("curl", "https://raw.githubusercontent.com/charlesreid1/five-letter-words/master/sgb-words.txt");


fn word_in_list(guess : &String) -> bool {
    for word in WORD_LIST.lines() {
        if word.to_string().eq(guess) {
            return true;
        }
    }
    return false;
}

fn read_guess() -> String {
    let mut guess = String::new();
    loop {
        let _bytes = std::io::stdin().read_line(&mut guess);
        guess = guess.trim().to_string();
        let characters = guess.chars().count();
        if characters == 5 {
            if !word_in_list(&guess) {
                println!("Not a valid word.");
            } else {
                break;
            }
        } else {
            println!("Incorrect number of bytes {}. Enter new guess.", characters);
        }
        guess = String::new();
    }
    return guess;
}

struct GuessResult {
    correct : bool,
    detail : String
}

fn check_word(word : String, guess : String) -> GuessResult {
    let mut output = String::new();
    let mut index = 0;
    let mut score = 0;
    for c in guess.chars() {
        if word.contains(c) {
            if word.chars().position(|ch| ch == c).unwrap() == index {
                output += "i";
                score += 1;
            } else {
                output += "c";
            }
        } else {
            output += "n";
        }
        index += 1;
    }
    return GuessResult{correct : (score == word.chars().count()), detail : output};
}


fn get_number(min : u64, max : u64) -> u64 {
    extern crate rand_chacha;
    extern crate rand_core;
    use rand_chacha::rand_core::SeedableRng;
    use rand_core::RngCore;

    use std::time::{SystemTime, UNIX_EPOCH};

    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");

    
    let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(since_the_epoch.as_secs()/3600/24);
    return rng.next_u64()%(max - min) + min;
}

fn get_word() -> &'static str {
    let words = Vec::from_iter(WORD_LIST.lines());
    let n = get_number(0, words.len() as u64) as usize;
    return words[n];
}

fn main() {
    println!("Guess the 5 letter word in 5 guesses!");
    println!("Hints:\n\t<n>: Letter is not in word.\n\t<c> Letter is in word.\n\t<i> Letter is at correct location");
    println!("Now enter your guess.");
    let word = get_word();
    for _n in 0..5 {
        let result = check_word(word.to_string(), read_guess());
        println!("{}", result.detail);
        if result.correct {
            println!("You guessed correctly.");
            return;
        }
    }
    println!("Try Again tomorrow. word was: '{}'", word);
}
