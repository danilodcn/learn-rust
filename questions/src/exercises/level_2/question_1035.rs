
pub mod question_1035 {
    use crate::shared::shared::{parse, input};
    pub fn resolve(input: String) -> &'static str {
        let numbers = parse::<i32>(&input);
        let a = numbers[0][0];
        let b = numbers[0][1];
        let c = numbers[0][2];
        let d = numbers[0][3];
        let message;
        if b > c && d > a && (c + d) > (a + b) && (c >= 0 && d >= 0) && a % 2 == 0 {
            message = "Valores aceitos";
        }
        else {
            message = "Valores nao aceitos";
        }

        return message
    }

    pub fn handle() {
        let inp = input();
        let message = resolve(inp);

        println!("{}", message);
    }
}

#[cfg(test)]
mod test {
    use super::question_1035::resolve;
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