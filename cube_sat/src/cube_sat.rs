#[derive(Debug)]
pub struct CubeSat {
    pub id: u64,
    mailbox: Mailbox,
}

#[derive(Debug)]
pub struct Mailbox {
    messages: Vec<Message>,
}

#[derive(Debug)]
pub struct Message {
    to: u64,
    content: String,
}

pub struct GroundStation;

impl CubeSat {
    pub fn new(id: u64) -> Self {
        Self {
            id,
            mailbox: Mailbox::new(),
        }
    }

    pub fn recv(&self, mailbox: &mut Mailbox) -> Option<Message> {
        mailbox.deliver(self)
    }
}

impl Mailbox {
    pub fn new() -> Self {
        Self { messages: vec![] }
    }

    fn post(&mut self, msg: Message) {
        self.messages.push(msg);
    }

    fn deliver(&mut self, recipient: &CubeSat) -> Option<Message> {
        for i in 0..self.messages.len() {
            if self.messages[i].to == recipient.id {
                let msg = self.messages.remove(i);
                return Some(msg);
            }
        }

        None
    }
}

impl Message {
    fn new(to: u64, content: String) -> Self {
        Self { to, content }
    }
}

impl GroundStation {
    pub fn connect(&self, sat_id: u64) -> CubeSat {
        CubeSat::new(sat_id)
    }

    pub fn send(&self, mailbox: &mut Mailbox, to: u64, msg: String) {
        let message = Message::new(to, msg);
        mailbox.post(message);
    }
}
