
use std::io::{Error, Write};

use sider_command::RESPType;


pub fn serialize<O: Write>(v: &RESPType, output: &mut O) -> Result<(), Error> {
    match v {
        RESPType::SimpleString(s) => serialize_simple_string(s, output),
        RESPType::Error(s) => serialize_error(s, output),
        RESPType::Integer(n) => serialize_integer(n, output),
        RESPType::BulkString(s) => serialize_bulk_string(s, output),
        RESPType::Array(a) => serialize_array(a, output),
        RESPType::Null => serialize_null(output),
    }
}


fn serialize_simple_string<O: Write>(v: &[u8], output: &mut O) -> Result<(), Error> {
    output.write_all(b"+")?;
    output.write_all(v)?;
    output.write_all(b"\r")?;
    output.write_all(b"\n")
}


fn serialize_error<O: Write>(v: &str, output: &mut O) -> Result<(), Error>  {
    output.write_all(b"-")?;
    output.write_all(v.as_bytes())?;
    output.write_all(b"\r")?;
    output.write_all(b"\n")
}


fn serialize_integer<O: Write>(v: &i64, output: &mut O) -> Result<(), Error> {
    output.write_all(b":")?;
    output.write_all(v.to_string().as_bytes())?;
    output.write_all(b"\r")?;
    output.write_all(b"\n")
} 


fn serialize_bulk_string<O: Write>(v: &[u8], output: &mut O) -> Result<(), Error> {
    output.write_all(b"$")?;
    output.write_all(v.len().to_string().as_bytes())?;
    output.write_all(b"\r")?;
    output.write_all(b"\n")?;

    output.write_all(v)?;
    output.write_all(b"\r")?;
    output.write_all(b"\n")
}


fn serialize_array<O: Write>(v: &[RESPType], output: &mut O) -> Result<(), Error> {
    output.write_all(b"*")?;
    output.write_all(v.len().to_string().as_bytes())?;
    output.write_all(b"\r")?;
    output.write_all(b"\n")?;

    for value in v {
        serialize(value, output)?;
    }

    Ok(())
}


fn serialize_null<O: Write>(output: &mut O) -> Result<(), Error> {
    output.write_all(b"$")?;
    output.write_all(b"-1")?;
    output.write_all(b"\r")?;
    output.write_all(b"\n")
}