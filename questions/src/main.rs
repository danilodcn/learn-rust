use std::collections::HashMap;

use clap::Parser;


pub mod exercises;
pub mod shared;

/// Programa para aprendizado da linguagem rust
#[derive(clap::Parser, Debug)]
struct Args {
    /// numero da questão resolvida
    question: String
}

fn main() {
    let args = Args::parse();

    let questions = get_questions();

    match questions.get(&args.question) {
        Some(f) => f(),
        None => {
            eprintln!("Exercício não resolvido! Os exercícios resolvidos são: {:?}", questions.keys().map(|x| &**x).collect::<Vec<_>>().join(", "))
        }
    }
}

use exercises::level_2::{question_1009, question_1035};
use exercises::level_6::question_1021;


fn get_questions() -> HashMap<String, fn()> {
    let mut functions: HashMap<String, fn()> = HashMap::new();

    functions.insert(String::from("1009"), question_1009::handle);
    functions.insert(String::from("1035"), question_1035::handle);
    functions.insert(String::from("1021"), question_1021::handle);
    
    return functions
}