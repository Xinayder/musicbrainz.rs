use uuid::ParseError;

#[derive(Debug)]
pub enum MBError {
    UuidParse(ParseError)
}