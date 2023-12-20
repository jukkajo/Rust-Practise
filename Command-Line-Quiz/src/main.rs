mod quiz;
mod util;

use std::io;

use std::process;
use quiz::practice_problems_random_order;
use quiz::read_problems_from_csv;

fn main() -> Result<(), io::Error> {

    let problems = read_problems_from_csv("src/resources/problems.csv")
        .unwrap_or_else(|err| {
            eprintln!("Error reading problems from CSV: {}", err);
            process::exit(1);
        });

    let mut problems = problems
        .iter()
        .map(|(q, a)| (q.as_str(), a.as_str()))
        .collect::<Vec<_>>();

    practice_problems_random_order(&mut problems);

    Ok(())
}
