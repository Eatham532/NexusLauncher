struct Message {
    pub __cdata: String,
}

struct Event {
    pub message: Message,
}

struct Throwable {
    pub message: Option<Message>,
    pub throwable: Option<Message>,
}

struct XmlLog {
    pub event: Event,
}