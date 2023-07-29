pub mod players;

pub enum ParserError {
    FileNotFound,
    CannotParseFile,
    ErrorWhileReading,
}
