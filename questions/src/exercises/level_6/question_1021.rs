use crate::shared::shared::{input, parse};

const BILLS: &'static [f64] = &[100., 50., 20., 10., 5., 2.];
const COINS: &'static [f64] = &[1., 0.50, 0.25, 0.10, 0.05, 0.01];


fn process(value: f64, input: f64) -> (i32, f64) {
    let mut div: f64 ;
    let mut count: i32 = 1;
    loop {
        div = input / (value * count as f64);
        if div < 1.0 {
            count -= 1;
            return (count, format!("{:.2}", input - count as f64 * value).parse::<f64>().unwrap())
        }
        count += 1;
    }
}


pub fn resolve(input: f64) -> Vec<String> {
    let mut number_of_notes: i32;
    let mut input = input;
    let mut res: Vec<String> = Vec::new();
    res.push(String::from("NOTAS:"));
    for value in BILLS {
        (number_of_notes, input) = process(*value, input);
        res.push(format!("{} nota(s) de R$ {:.2}", number_of_notes, value));
    }
    res.push(String::from("MOEDAS:"));

    for value in COINS {
        (number_of_notes, input) = process(*value, input);
        res.push(format!("{} moeda(s) de R$ {:.2}", number_of_notes, value));
    }
    return res;
}

pub fn handle() {
    println!("Insira um número: ");
    let value = parse::<f64>(&input());
    let messages = resolve(value[0][0]);

    println!("{}", messages.join("\n"))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_process_with_2_dollars() {
        let value = 2.0;
        let input = 13.0;
        let expected = (6, 1.0);
        let res = process(value, input);
        assert_eq!(expected, res);
    }

    #[test]
    fn test_process_with_30_cents() {
        let value = 0.3;
        let input = 0.99;
        let expected = (3, 0.09);
        let res = process(value, input);
        assert_eq!(expected, res);
    }

    #[test]
    fn test_1021_01() {
        let input = 576.73;
        let response = resolve(input);
        let expected = ["NOTAS:",
        "5 nota(s) de R$ 100.00",
        "1 nota(s) de R$ 50.00",
        "1 nota(s) de R$ 20.00",
        "0 nota(s) de R$ 10.00",
        "1 nota(s) de R$ 5.00",
        "0 nota(s) de R$ 2.00",
        "MOEDAS:",
        "1 moeda(s) de R$ 1.00",
        "1 moeda(s) de R$ 0.50",
        "0 moeda(s) de R$ 0.25",
        "2 moeda(s) de R$ 0.10",
        "0 moeda(s) de R$ 0.05",
        "3 moeda(s) de R$ 0.01"];

        assert_eq!(response, expected)
    }

    #[test]
    fn test_1021_02() {
        let input = 4.00;
        let response = resolve(input);
        let expected = ["NOTAS:",
        "0 nota(s) de R$ 100.00",
        "0 nota(s) de R$ 50.00",
        "0 nota(s) de R$ 20.00",
        "0 nota(s) de R$ 10.00",
        "0 nota(s) de R$ 5.00",
        "2 nota(s) de R$ 2.00",
        "MOEDAS:",
        "0 moeda(s) de R$ 1.00",
        "0 moeda(s) de R$ 0.50",
        "0 moeda(s) de R$ 0.25",
        "0 moeda(s) de R$ 0.10",
        "0 moeda(s) de R$ 0.05",
        "0 moeda(s) de R$ 0.01"];
        
        assert_eq!(response, expected)
    }

    #[test]
    fn test_1021_03() {
        let input = 91.01;
        let response = resolve(input);
        let expected = ["NOTAS:",
        "0 nota(s) de R$ 100.00",
        "1 nota(s) de R$ 50.00",
        "2 nota(s) de R$ 20.00",
        "0 nota(s) de R$ 10.00",
        "0 nota(s) de R$ 5.00",
        "0 nota(s) de R$ 2.00",
        "MOEDAS:",
        "1 moeda(s) de R$ 1.00",
        "0 moeda(s) de R$ 0.50",
        "0 moeda(s) de R$ 0.25",
        "0 moeda(s) de R$ 0.10",
        "0 moeda(s) de R$ 0.05",
        "1 moeda(s) de R$ 0.01"];
        
        assert_eq!(response, expected)
    }
}