use crate::Position;

pub enum Message {
    SetPiece(Position),
    TextMessage(String),
    PassTurn(),
    Surrender(),
}

impl Message {
    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            Self::SetPiece(pos) => format!("3{}{}{}", "0", pos.0, pos.1),
            Self::TextMessage(text) => format!("{}{}{}", text.len(), "1", text),
            Self::PassTurn() => "12".to_string(),
            Self::Surrender() => "13".to_string(), 
        }.into_bytes()
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, &'static str> {
        match bytes.get(0).expect("Cannot read from an empty byte string.") {
            b'0' => {
                let x = bytes.get(1).expect("Missing X value for position message.");
                let y = bytes.get(2).expect("Missing Y value for position message.");
                Ok(Message::SetPiece((*x as usize, *y as usize)))
            },
            b'1' => {
                let text = String::from_utf8(bytes[1..].to_vec()).unwrap();
                Ok(Message::TextMessage(text))
            }
            b'2' => Ok(Message::PassTurn()),
            b'3' => Ok(Message::Surrender()),
            _ => Err("Non-existant message type received")
        }
    }
}
