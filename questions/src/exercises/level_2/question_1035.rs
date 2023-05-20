
use crate::shared::shared::{parse, input};
pub fn resolve(input: String) -> &'static str {
    let message;
    let numbers = parse::<i32>(&input);
    
    if let [a, b, c, d] = numbers[0][..] {
        if b > c && d > a && (c + d) > (a + b) && (c >= 0 && d >= 0) && a % 2 == 0 {
            message = "Valores aceitos";
        }
        else {
            message = "Valores nao aceitos";
        }
        return message
    }
    else {
        panic!("Erro no parser dos dados!")
    }
}

pub fn handle() {
    let inp = input();
    let message = resolve(inp);

    println!("{}", message);
}

#[cfg(test)]
mod test {
    use super::resolve;
    #[test]
    fn test_01() {
        let expected = "Valores nao aceitos";
        let input = String::from("5 6 7 8");
        assert_eq!(resolve(input), expected)
    }
    #[test]
    fn test_02() {
        let expected = "Valores aceitos";
        let input = String::from("2 3 2 6");
        assert_eq!(resolve(input), expected)
    }
}