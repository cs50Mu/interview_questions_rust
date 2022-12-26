use anyhow::Result;

fn ipv4_to_int(s: &str) -> Result<u32> {
    let ints: Vec<u32> = s
        .split('.')
        .map(|s| -> Result<u32> {
            let s = s.trim();
            // valid ipv4 in dot-decimal notation
            // every decimal number should be within u8
            let i = s.parse::<u8>()?;
            Ok(i as u32)
        })
        .collect::<Result<Vec<u32>>>()?;

    let (one, two, three, four) = (ints[0] << 24, ints[1] << 16, ints[2] << 8, ints[3]);

    Ok(one | two | three | four)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_addr_should_work() {
        let s = "192.0.2.235";
        let res = ipv4_to_int(s);
        assert_eq!(res.unwrap(), 3221226219_u32)
    }

    #[test]
    fn addr_with_whitespace_should_work() {
        let s = "192 .0 .2. 235";
        let res = ipv4_to_int(s);
        assert_eq!(res.unwrap(), 3221226219_u32)
    }

    #[test]
    fn malformed_addr_should_return_err() {
        let s = "xyz .i .2. 768";
        let res = ipv4_to_int(s);
        assert!(res.is_err())
    }

    #[test]
    fn invalid_addr_should_return_err() {
        let s = "123.456.789.000";
        let res = ipv4_to_int(s);
        assert!(res.is_err())
    }
}
