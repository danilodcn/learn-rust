use clap::Parser;


mod questions;
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

    let x = shared::shared::input::<i32>();
    println!("{:?}", x);
}
