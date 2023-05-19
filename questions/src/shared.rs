pub mod shared {
    use std::{self, str, io, fmt, cmp};
    pub fn input<T>() -> Vec<Vec<T>>
    where T: str::FromStr + fmt::Debug + cmp::PartialEq, <T as str::FromStr>::Err : fmt::Debug
    {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        println!("line {}", line);
        return self::parse(&line);
        // value = line.parse::<T>().map(|x| x);
    }

    pub fn parse<T>(string: &String) -> Vec<Vec<T>>
    where T: str::FromStr + fmt::Debug + cmp::PartialEq, <T as str::FromStr>::Err : fmt::Debug
    {
        let strings: Vec<Vec<T>> = string.split("\n").map(|s| {
            s.split(" ").filter(|x| {
                if *x == "" { false }
                else { true }
            })
            .map(|x| {
                let y = match x.parse::<T>() {
                    Err(_) => None,
                    Ok(v) => Some(v)
                };
                let z = y.expect(&format!("não foi possível converter '{x}'"));
                z
            }).collect()
        }).filter(|v: &Vec<T>| {
            if v.len() == 0 { false }
            else { true }
        }).collect();


        println!("ola {:?}", strings);
        return strings
    }
}

#[cfg(test)]
mod test {
    use crate::shared::shared::parse;

    #[test]
    fn test_parse_string_in_vector_with_new_line() {
        let input = String::from("20 30 40\n");
        assert_eq!(parse::<i32>(&input), [[20, 30, 40]]);
    }
    #[test]
    fn test_parse_string_in_vector_without_new_line() {
        let input = String::from("20 30 40");
        assert_eq!(parse::<i32>(&input), [[20, 30, 40]]);
    }
    #[test]
    fn test_parse_string_with_many_lines_in_vector() {
        let input = String::from("20 30 40\n 40 30 20\n25");
        assert_eq!(parse::<i32>(&input), vec![vec![20, 30, 40], vec![40, 30, 20], vec![25]]);
    }
}
