use crate::shared::shared::input;

pub fn resolve(inputs: &[String]) -> String {
    if let [_, salary_input, sales_input] = inputs {
        let salary = salary_input.parse::<f64>().unwrap();
        let sales = sales_input.parse::<f64>().unwrap();

        const COMMISSION_PERCENTAGE: f64 = 15.0;
        
        let value = salary + sales * COMMISSION_PERCENTAGE / 100.0;

        return format!("TOTAL = R$ {:.2}", value);
    }
    else {
        panic!("erro no parse!")
    };
}

pub fn handle() {
    let name = input();
    let salary = input();
    let sales = input();
    let message = resolve(&[name, salary, sales]);

    println!("{}", message);
}


#[cfg(test)]
mod test {
    use super::resolve;
    #[test]
    fn test_01() {
        let message = resolve(&["Danilo", "500.00", "1230.30"].map(|x| String::from(x)));
        let expected = "TOTAL = R$ 684.54";

        assert_eq!(message, expected)
    }
    
    #[test]
    fn test_02() {
        let message = resolve(&["PEDRO", "700.00", "0.00"].map(|x| String::from(x)));
        let expected = "TOTAL = R$ 700.00";

        assert_eq!(message, expected)
    }

    #[test]
    fn test_03() {
        let message = resolve(&["DANILO", "1700.00", "1230.50"].map(|x| String::from(x)));
        let expected = "TOTAL = R$ 1884.58";

        assert_eq!(message, expected)
    }
}