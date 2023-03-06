pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

pub fn parse_number(code: &String, idx: &mut usize) -> i32 {
    let mut c;
    let mut number_str = String::new();
    loop {
        c = code.chars().nth(*idx).unwrap();
        if c.is_numeric() {
            number_str.push(c);
        } else {
            break;
        }

        if *idx == code.len() - 1 {
            break;
        } else {
            *idx += 1;
        }
    }
    return number_str.parse::<i32>().unwrap();
}

pub fn ignore_whitespace(code: &String, idx: &mut usize) {
    loop {
        if let Some(c) = code.chars().nth(*idx) {
            if c == ' ' {
                *idx += 1;
                continue;
            } else {
                return;
            }
        } else {
            return;
        }
    }
}

pub fn parse_expression(code: &String, idx: &mut usize) -> i32 {
    let mut lhs = parse_number(&code, idx);
    loop {
        let op: Op;
        ignore_whitespace(&code, idx);
        if let Some(c) = code.chars().nth(*idx) {
            op = match c {
                '+' => Op::Add,
                '-' => Op::Sub,
                '*' => Op::Mul,
                '/' => Op::Div,
                _ => panic!("unexpected operator"),
            };
            *idx += 1;
            ignore_whitespace(&code, idx);
            let rhs = parse_number(&code, idx);
            lhs = match op {
                Op::Add => lhs + rhs,
                Op::Sub => lhs - rhs,
                Op::Mul => lhs * rhs,
                Op::Div => lhs / rhs,
            };
            if *idx == code.len() - 1 {
                break;
            } else {
                *idx += 1;
            }
        } else {
            break;
        }
    }
    return lhs;
}

#[cfg(test)]
mod tests {

    use crate::parse_number;
    use crate::ignore_whitespace;
    use crate::parse_expression;

    #[test]
    fn parse_test() {
        let mut idx: usize = 0;
        let c = parse_number(&"123abc".to_string(), &mut idx);
        assert_eq!(c, 123);
        assert_eq!(idx, 3);
    }

    #[test]
    fn test_ignore_whitespace() {
        let mut idx: usize = 0;
        ignore_whitespace(&"  abc".to_string(), &mut idx);
        assert_eq!(idx, 2);
    }

    #[test]
    fn test_parse_expression() {
        let mut idx: usize = 0;
        let res = parse_expression(&"1 + 2 + 1".to_string(), &mut idx);
        assert_eq!(res, 4);
        let mut idx: usize = 0;
        let res = parse_expression(&"100 + 2  - 2 ".to_string(), &mut idx);
        assert_eq!(res, 100);
    }
}
