use crate::Position;

pub const BUFFER_SIZE: usize = 256;

pub enum Message {
    SetPiece(Position),
    TextMessage(String),
    PassTurn(),
    Surrender(),
    GameEnded(),
    UndoMove(),
    TestConnection()
}

impl Message {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = match self {
            Self::SetPiece(pos) => format!("{}{}{}", "0", pos.0, pos.1),
            Self::TextMessage(text) => format!("{}{}", "1", text),
            Self::PassTurn() => "2".to_string(),
            Self::Surrender() => "3".to_string(),
            Self::GameEnded() => "4".to_string(),
            Self::UndoMove() => "5".to_string(),
            Self::TestConnection() => "6".to_string(),
        }.into_bytes();

        bytes.resize(BUFFER_SIZE, 0); // pads the buffer with NULL characters
        bytes
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, &'static str> {
        match bytes.get(0).expect("Cannot read from an empty byte string.") {
            b'0' => {
                let x = bytes.get(1).expect("Missing X value for position message.");
                let y = bytes.get(2).expect("Missing Y value for position message.");
                Ok(Message::SetPiece((*x as usize - 0x30, *y as usize - 0x30))) // subtracts '0' ascii
            },
            b'1' => {
                let text = String::from_utf8(bytes[1..].to_vec()).unwrap();
                Ok(Message::TextMessage(text))
            }
            b'2' => Ok(Message::PassTurn()),
            b'3' => Ok(Message::Surrender()),
            b'4' => Ok(Message::GameEnded()),
            b'5' => Ok(Message::UndoMove()),
            _ => Err("Non-existant message type received")
        }
    }
}
