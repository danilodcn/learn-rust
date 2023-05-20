use crate::shared::shared::{input, parse};

pub fn resolve(inputs: &[String]) -> String {
    let values = parse::<f64>(&inputs[1..].join(" "));
    let salary = values[0][0];
    let sales = values[1][0];
    const COMMISSION_PERCENTAGE: f64 = 15.0;
    
    let value = salary + sales * COMMISSION_PERCENTAGE / 100.0;

    return format!("TOTAL = R$ {:.2}", value);
}

pub fn handle() {
    println!("Nome: ");
    let name = input();
    println!("Salário: ");
    let salary = input();
    println!("Vendas: ");
    let sales = input();
    let message = resolve(&[name, salary, sales]);

    println!("{}", message);
}


#[cfg(test)]
mod test {
    use super::resolve;
    #[test]
    fn test_01() {
        let message = resolve(&["Danilo\n", "500.00\n", "1230.30\n"].map(|x| String::from(x)));
        let expected = "TOTAL = R$ 684.54";

        assert_eq!(message, expected)
    }
    
    #[test]
    fn test_02() {
        let message = resolve(&["PEDRO\n", "700.00\n", "0.00\n"].map(|x| String::from(x)));
        let expected = "TOTAL = R$ 700.00";

        assert_eq!(message, expected)
    }

    #[test]
    fn test_03() {
        let message = resolve(&["DANILO\n", "1700.00\n", "1230.50\n"].map(|x| String::from(x)));
        let expected = "TOTAL = R$ 1884.58";

        assert_eq!(message, expected)
    }
}