

use bytes::Bytes;
use log::debug;

use sider_command::RESPType;


#[derive(Default)]
pub struct RESPParser {
    position: usize,
    bytes: Bytes
}


impl RESPParser {
    pub fn parse(s: Bytes) -> RESPType<Bytes> {
        let mut parser = Self {
            position: 0,
            bytes: s,
        };

        parser.parse_until_complete()
    }

    fn parse_until_complete(&mut self) -> RESPType<Bytes> {
        debug!("Parse until complete.");

        if let Some(first_byte) = self.bytes.get(self.position) {
            debug!("First byte was {:?}", char::from_u32(*first_byte as u32).unwrap());
            self.position += 1;

            match first_byte {
                b'+' => self.parse_simple_string(),
                b'*' => self.parse_array(),
                b'$' => self.parse_bulk_string(),
                b':' => self.parse_integer(),
                b'-' => self.parse_error(),
                _ => RESPType::Error("Unable to parse input due to invalid byte.".into()),
            }
        } else {
            RESPType::Error("Unable to parse, no byte at position.".into())
        }
    }
    
    fn read_until(&mut self, c: u8) {
        if let Some(index) = self.bytes[self.position..].iter().position(|b| b == &c) {
            debug!("Setting position with index {}", index);
            self.position = self.position + index;
        } else {
            debug!("Char not found, exiting.");
            self.position = self.bytes.len();
        }
    }

    fn parse_simple_string(&mut self) -> RESPType<Bytes> {
        debug!("Parsing simple string.");

        let start = self.position;

        self.read_until(b'\r');
        
        self.position += 1;

        if Some(&b'\n') != self.bytes.get(self.position) {
            return RESPType::Error("Missing newline.".into())
        }

        self.position += 1;

        RESPType::SimpleString(self.bytes.slice(start..self.position - 2).into())
    }

    fn parse_integer(&mut self) -> RESPType<Bytes> {
        debug!("Parsing integer.");

        let start = self.position;

        self.read_until(b'\r');
        
        self.position += 1;

        if Some(&b'\n') != self.bytes.get(self.position) {
            return RESPType::Error("Missing newline.".into())
        }

        self.position += 1;
        
        if let Ok(parsed_integer_string) = std::str::from_utf8(&self.bytes[start..self.position - 2]) {
            if let Ok(v) = str::parse::<i64>(parsed_integer_string) {
                RESPType::Integer(v)
            } else {
                RESPType::Error("Unable to parse integer, string is not an integer.".into())
            }
        } else {
            RESPType::Error("Unable to parse integer, invalid byte sequence.".into())
        }

    }

    fn parse_error(&mut self) -> RESPType<Bytes> {
        debug!("Parsing error.");

        let start = self.position;

        self.read_until(b'\r');
        
        self.position += 1;

        if Some(&b'\n') != self.bytes.get(self.position) {
            return RESPType::Error("Missing newline.".into())
        }

        self.position += 1;
        
        RESPType::Error(self.bytes.slice(start..self.position - 2))
    }

    fn parse_array(&mut self) -> RESPType<Bytes> {
        debug!("Parsing array.");

        let start = self.position;
        debug!("Start: {}", start);

        self.read_until(b'\r');

        // Check for newline
        self.position += 1;

        if Some(&b'\n') != self.bytes.get(self.position) {
            return RESPType::Error("Missing newline.".into())
        }

        debug!("Got to position {}", self.position);
        
        let array_length = if let Ok(parsed_length_string) = std::str::from_utf8(&self.bytes[start..self.position - 1]) {
            if let Ok(v) = str::parse::<i64>(parsed_length_string) {
                v
            } else {
                return RESPType::Error("Unable to parse array length, string not an integer.".into());
            }
        } else {
            return RESPType::Error("Unable to parse array length, invalid byte sequence.".into());
        };

        debug!("Parsed length: {}", array_length);

        let mut items = vec![];

        if array_length == -1 {
            return RESPType::Null;
        }

        let array_length = if let Ok(v) = usize::try_from(array_length) {
            v
        } else {
            return RESPType::Error("Invalid array length, length was negative and not -1.".into());
        };

        self.position += 1;

        while items.len() < array_length {
            debug!("Parsing item {}", items.len());

            let item = self.parse_until_complete();

            if let RESPType::Error(_) = item {
                debug!("Got error for item. Returning.");
                return item;
            }

            items.push(item);
        }

        RESPType::Array(items)
    }

    fn parse_bulk_string(&mut self) -> RESPType<Bytes> {
        debug!("Parsing bulk string.");

        let start = self.position;
        debug!("Start: {}", start);

        self.read_until(b'\r');

        // Check for newline
        self.position += 1;

        if Some(&b'\n') != self.bytes.get(self.position) {
            return RESPType::Error("Missing newline.".into())
        }

        debug!("Got to position {}", self.position);
        
        let string_length = if let Ok(parsed_length_string) = std::str::from_utf8(&self.bytes[start..self.position - 1]) {
            if let Ok(v) = str::parse::<i64>(parsed_length_string) {
                v
            } else {
                return RESPType::Error("Unable to parse string length, invalid integer".into());
            }
        } else {
            return RESPType::Error("Unable to parse string length, invalid byte sequence.".into());
        };

        debug!("Parsed length: {}", string_length);

        if string_length == -1 {
            return RESPType::Null;
        }

        let string_length = if let Ok(v) = usize::try_from(string_length) {
            v
        } else {
            return RESPType::Error("Unable to parse bulk string length, integer is negative and not -1.".into());
        };

        let string_start = self.position + 1;
        self.position += 1 + string_length;

        if Some(&b'\r') != self.bytes.get(self.position) {
            return RESPType::Error("Unable to parse bulk string, invalid string length.".into())
        }

        self.position += 1;

        if Some(&b'\n') != self.bytes.get(self.position) {
            return RESPType::Error("Unable to parse bulk string, missing newline.".into())
        }

        self.position += 1;

        RESPType::BulkString(self.bytes.slice(string_start..self.position - 2).into())
    }
}



#[cfg(test)]
mod tests {
    use crate::parser::{RESPParser, RESPType};

    #[test]
    fn test_null() {
        assert_eq!(RESPParser::parse("$-1\r\n".into()), RESPType::Null);
    }

    #[test]
    fn test_array() {
        assert_eq!(
            RESPParser::parse("*1\r\n$4\r\nping\r\n".into()),
            RESPType::Array(
                vec![RESPType::BulkString("ping".into())]
            )
        )
    }

    #[test]
    fn test_array_multiple_itemse() {
        assert_eq!(
            RESPParser::parse("*2\r\n$4\r\necho\r\n$11\r\nhello world\r\n".into()),
            RESPType::Array(
                vec![
                    RESPType::BulkString("echo".into()),
                    RESPType::BulkString("hello world".into()),
                ]
            )
        )
    }

    #[test]
    fn test_simple_string() {
        assert_eq!(
            RESPParser::parse("+OK\r\n".into()),
            RESPType::SimpleString("OK".into())
        )
    }

    #[test]
    fn test_error() {
        assert_eq!(
            RESPParser::parse("-Error message\r\n".into()),
            RESPType::Error("Error message".into())
        )
    }

    #[test]
    fn test_empty_bulk_string() {
        assert_eq!(
            RESPParser::parse("$0\r\n\r\n".into()),
            RESPType::BulkString("".into())
        )
    }

    #[test]
    fn test_hello_world() {
        assert_eq!(
            RESPParser::parse("+hello world\r\n".into()),
            RESPType::SimpleString("hello world".into())
        )
    }


    #[test]
    fn test_invalid_input() {
        assert_eq!(
            RESPParser::parse("bad string".into()),
            RESPType::Error("Unable to parse input due to invalid byte.".into()),
        )

    }
}

