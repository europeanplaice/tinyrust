pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

pub fn consume_number(code: &String, idx: &mut usize) -> i32 {
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
    let mut c;
    loop {
        c = code.chars().nth(*idx).unwrap();
        if c == ' ' {
            *idx += 1;
            continue;
        } else {
            return;
        }
    }
}

pub fn parse_expression(code: &String, idx: &mut usize) -> i32 {
    let rhs = consume_number(&code, idx);
    let c;
    let op: Op;
    ignore_whitespace(&code, idx);
    c = code.chars().nth(*idx).unwrap();
    op = match c {
        '+' => Op::Add,
        '-' => Op::Sub,
        '*' => Op::Mul,
        '/' => Op::Div,
        _ => panic!("unexpected operator"),
    };
    *idx += 1;
    ignore_whitespace(&code, idx);
    let lhs = consume_number(&code, idx);
    let res = match op {
        Op::Add => rhs + lhs,
        Op::Sub => rhs - lhs,
        Op::Mul => rhs * lhs,
        Op::Div => rhs / lhs,
    };
    return res;
}

#[cfg(test)]
mod tests {

    use crate::consume_number;
    use crate::ignore_whitespace;
    use crate::parse_expression;

    #[test]
    fn parse_test() {
        let mut idx: usize = 0;
        let c = consume_number(&"123abc".to_string(), &mut idx);
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
        let res = parse_expression(&"1 + 2".to_string(), &mut idx);
        assert_eq!(res, 3);
        let mut idx: usize = 0;
        let res = parse_expression(&"100 + 2   ".to_string(), &mut idx);
        assert_eq!(res, 102);
    }
}