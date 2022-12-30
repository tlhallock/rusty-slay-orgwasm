// enum MessageType {
//   Notification,
// }

// struct Message {
//   message_type: MessageType,
//   message_text: String,
// }

pub struct Notification {
    pub message_text: String,
}

impl Notification {
    pub fn new(message_text: String) -> Self {
        Notification { message_text }
    }
}

impl From<&'static str> for Notification {
    fn from(value: &'static str) -> Self {
        Notification {
            message_text: value.to_string(),
        }
    }
}
