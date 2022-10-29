use std::array::TryFromSliceError;
use std::error::Error;
use std::fmt::Display;

pub type RawMessage = [u8; 9];

#[derive(Debug)]
pub struct MessageError {}

impl Display for MessageError {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}

impl Error for MessageError {}

impl From<TryFromSliceError> for MessageError {
    fn from(_: TryFromSliceError) -> Self {
        MessageError {}
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Message {
    Insert { timestamp: i32, price: i32 },
    Query { min_time: i32, max_time: i32 },
    Undefined,
}

fn parse_arg(v: &[u8]) -> Result<i32, MessageError> {
    let casted = v.try_into()?;
    Ok(i32::from_be_bytes(casted))
}

impl TryFrom<RawMessage> for Message {
    type Error = MessageError;

    fn try_from(value: RawMessage) -> Result<Self, Self::Error> {
        let action = value[0];
        let first_arg: i32 = parse_arg(&value[1..5]).unwrap();
        let second_arg: i32 = parse_arg(&value[5..9]).unwrap();

        let parsed = match action {
            b'I' => Message::Insert {
                timestamp: first_arg,
                price: second_arg,
            },
            b'Q' => Message::Query {
                min_time: first_arg,
                max_time: second_arg,
            },
            _ => {
                return Err(MessageError {});
            }
        };

        Ok(parsed)
    }
}

#[test]
fn test_message_for_query() {
    let message = Message::Query {
        min_time: 0,
        max_time: 1000,
    };

    let frame = [b'Q', 0, 0, 0, 0, 0, 0, 3, 232];

    assert_eq!(Message::try_from(frame).unwrap(), message)
}

#[test]
fn test_message_for_invalid_data() {
    let frame = [0, 0, 0, 0, 0, 0, 0, 0, 0];
    assert!(Message::try_from(frame).is_err())
}
