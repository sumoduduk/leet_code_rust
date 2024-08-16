pub fn roman_to_int(s: String) -> i32 {
    let mut total = 0;

    let mut character = s.chars().peekable();

    fn extract(c: char) -> i32 {
        match c {
            'I' => 1,
            'V' => 5,
            'X' => 10,
            'L' => 50,
            'C' => 100,
            'D' => 500,
            'M' => 1000,
            _ => 0,
        }
    }

    while let Some(cha1) = character.next() {
        let cha2 = character.peek().unwrap_or(&'t');

        let num = match cha1 {
            'I' => match cha2 {
                'X' => {
                    character.next();
                    9
                }
                'V' => {
                    character.next();
                    4
                }
                _ => 1,
            },
            'X' => match cha2 {
                'L' => {
                    character.next();
                    40
                }
                'C' => {
                    character.next();
                    90
                }
                _ => 10,
            },
            'C' => match cha2 {
                'D' => {
                    character.next();
                    400
                }
                'M' => {
                    character.next();
                    900
                }
                _ => 100,
            },
            _ => extract(cha1),
        };

        total += num;
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roman_1() {
        let s = "III".to_string();
        let out = 3;

        assert_eq!(roman_to_int(s), out);
    }

    #[test]
    fn test_roman_2() {
        let s = "LVIII".to_string();
        let out = 58;

        assert_eq!(roman_to_int(s), out);
    }

    #[test]
    fn test_roman_3() {
        let s = "MCMXCIV".to_string();
        let out = 1994;

        assert_eq!(roman_to_int(s), out);
    }
}
