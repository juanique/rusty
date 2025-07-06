#![allow(unused)]

#[derive(Debug)]
enum StatusMessage {
    Ok,
}

#[derive(Debug)]
struct Message {
    to: u64,
    content: String,
}

#[derive(Debug)]
struct Mailbox {
    messages: Vec<Message>,
}

impl Mailbox {
    fn new() -> Self {
        Self { messages: vec![] }
    }

    fn post(&mut self, msg: Message) {
        self.messages.push(msg);
    }

    fn deliver(&mut self, recipient: &CubeSat) -> Option<Message>{
        for i in 0..self.messages.len() {
            if self.messages[i].to == recipient.id {
                let msg = self.messages.remove(i);
                return Some(msg);
            }
        }
        None
    }
}

#[derive(Debug)]
struct CubeSat {
    id: u64,
}

impl CubeSat {
    fn new(id: u64) -> Self {
        Self {
            id,
        }
    }

    fn recv(&self, mailbox: &mut Mailbox) -> Option<Message> {
        mailbox.deliver(&self)
    }
}

fn fetch_sat_ids() -> Vec<u64> {
    vec![0, 1, 2]
}

struct GroundStation;

impl GroundStation {
    fn send(&self, mailbox: &mut Mailbox, msg: Message) {
        mailbox.post(msg);
    }

    fn connect(&self, sat_id: u64) -> CubeSat {
        CubeSat::new(sat_id)
    }
}


fn main() {
    let mut mail = Mailbox::new();
    let base = GroundStation{};

    let sat_ids = fetch_sat_ids();
    for sat_id in sat_ids {
        let msg = Message { to: sat_id, content: String::from("hello") };
        base.send(&mut mail, msg);
    }

    let sat_ids = fetch_sat_ids();
    for sat_id in sat_ids {
        let sat = base.connect(sat_id);
        let msg = sat.recv(&mut mail);
        println!("{:?}: {:?}", sat, msg);
    }

}
