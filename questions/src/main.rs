use clap::Parser;


pub mod exercises;
pub mod shared;

/// Programa para aprendizado da linguagem rust
#[derive(clap::Parser, Debug)]
struct Args {
    /// numero da questão resolvida
    question: usize
}

fn main() {
    let args = Args::parse();
    println!("{:?}", args);

    let x = shared::shared::input();
    println!("{:?}", x);
}
