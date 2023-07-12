

pub fn from_decimal_bytes(b: &[u8]) -> Result<i64, ()> {    
    if b.len() == 0 {
        return Err(());
    }

    if b.len() == 1 && b[0] == b'0' {
        return Ok(0);
    }

    let mut start = 0;

    let is_negative = if b[0] == b'-' {
        if b.len() == 1 {
            return Err(())
        }

        start = 1;

        true
    } else {
        false
    };

    // We've already dealt with the case where b is the string b'0', so
    // this must be an invalid 
    if b[start] == b'0' {
        return Err(());
    }

    let mut result: u64 = 0;

    for byte in b {
        if *byte < b'0' || *byte > b'9' {
            return Err(());
        }

        result *= 10;
        result += (byte - b'0') as u64;
    }

    if is_negative {
        if result > -(i64::MIN + 1) as u64 {
            return Err(());
        }

        Ok(-(result as i64))
    } else if result > i64::MAX as u64 {
        Err(())
    } else {
        Ok(result as i64)
    }
}
