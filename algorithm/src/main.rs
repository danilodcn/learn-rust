use clap::Parser;
use algorithms::c1::{insertion_sort, merge_sort};

    
/// Implementação dos exemplos do livro Algorítimos usando a linguagem Rust
#[derive(clap::Parser, Debug)]
struct Args {
    /// algoritmo usado
    #[clap(value_enum)]
    algorithm: Algorithm,
    /// vetor a ordenar
    #[arg(short, long, default_value_t=String::new())]
    vector: String,
}

#[derive(clap::ValueEnum, Clone, Debug)]
enum Algorithm {
   Insertion,
   Merge,
}

fn main() {
    let args = Args::parse();
    println!("{:?}", args);
    // let mut default = [1.0, 5.0, 4.5];

    let mut arr = get_vector(&args);

    let func = match args.algorithm {
        Algorithm::Insertion => insertion_sort,
        Algorithm::Merge => merge_sort
    };
    func(&mut arr);

    println!("final: {:?}", arr);
}


fn get_vector(args: &Args) -> Vec<f64>{
    let r = args.vector.split(",")
    .map(|x| x.parse::<f64>().expect(&format!("não foi possível converter '{x}' em número")));

    let x = r.collect();
    return x
}