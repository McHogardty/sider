

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum RESPType {
    SimpleString(Vec<u8>),
    Error(String),
    Integer(i64),
    BulkString(Vec<u8>),
    Array(Vec<RESPType>),
    Null,
}
