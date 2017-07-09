pub fn calculate(s: &str) -> Result<i64, String> {
    let st = s.trim();

    if st.len() == 0 {
        return Err("Syntax error: no values.".to_owned());
    }

    let ops = st.split(" ");

    let mut stack: Vec<i64> = vec![];

    for o in ops {
        match o.parse::<i64>() {
            Ok(v) => {
                stack.push(v);
                continue;
            }
            Err(_) => (),
        }

        let a = match stack.pop() {
            Some(v) => v,
            None => return Err("Syntax error: too few values.".to_owned()),
        };

        let b = match stack.pop() {
            Some(v) => v,
            None => return Err("Syntax error: too few values.".to_owned()),
        };

        match o {
            "+" => {
                stack.push(a + b);
            }
            "-" => {
                stack.push(b - a);
            }
            "*" => {
                stack.push(a * b);
            }
            "/" => {
                stack.push(b / a);
            }
            _ => return Err(format!("Syntax error: {}", o)),
        }
    }

    return Ok(stack.last().unwrap().to_owned());
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
