use crate::util::read_input;
use rand::seq::SliceRandom;
use std::{error::Error, io, process};
use csv::ReaderBuilder;
use std::fs::File;

pub fn read_problems_from_csv(csv_filename: &str) -> Result<Vec<(String, String)>, io::Error> {
    let mut output = Vec::new();
    let file = File::open(csv_filename)?;
    let mut reader = ReaderBuilder::new().from_reader(file);
    
    for result in reader.records() {
        let record = result?;
        
        if let (Some(first_column), Some(second_column)) = (record.get(0), record.get(1)) {
            let first_column = first_column.to_string();
            let second_column = second_column.to_string();
            output.push((first_column, second_column));
        } else {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid CSV format"));
        }
    }
    
    Ok(output)
}

pub fn ask_question(question: &str, correct_answer: &str) -> bool {
    println!("{}", question);
    let user_answer = read_input();
    user_answer == correct_answer
}

pub fn practice_problems(problems: &[(&str, &str)]) {
    let mut score = 0;
    for (question, correct_answer) in problems {
        let correct = ask_question(question, correct_answer);
        if correct {
            score += 1;
            println!("Correct!");
        } else {
            println!("Incorrect! The correct answer is {}", correct_answer);
        }
    }
    println!("You got {} out of {} correct!", score, problems.len());
}

pub fn practice_problems_random_order(problems: &mut [(&str, &str)]) {
    let mut rng = rand::thread_rng();
    problems.shuffle(&mut rng);
    practice_problems(problems);
}

fn main() {
    if rand::random() {

        println!("char: {}", rand::random::<char>());
    }
}

