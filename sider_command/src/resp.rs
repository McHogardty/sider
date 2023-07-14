

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum RESPType<T> {
    SimpleString(T),
    Error(T),
    Integer(i64),
    BulkString(T),
    Array(Vec<RESPType<T>>),
    Null,
}


impl RESPType<Vec<u8>> {
    pub fn as_ref(&self) -> RESPType<&[u8]> {
        RESPType::Null
    }
}