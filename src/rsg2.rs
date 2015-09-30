extern crate time;

use std::collections::HashMap;

/// A map of Channels
pub struct StreamMap <'a> {
    channels: HashMap<&'a str, Channel>
}

impl <'a> StreamMap <'a> {
    pub fn new () -> StreamMap <'a> {
        StreamMap {
            channels: HashMap::new()
        }
    }

    /// Adds a new Channel to the map
    ///
    pub fn add_channel (&mut self, topic: &str) -> Channel {
        Channel::new(topic)
    }
}

enum Snake {

}

pub struct Channel {
    topic: String,
    ttl: i32,
    messages: Vec<EventEnvelope>
}

impl Channel {

    pub fn new (channel_name: &str) -> Channel {
        Channel {
            topic: channel_name.to_string(),
            ttl: 30,
            messages: Vec::new()
        }
    }

    pub fn add_message (&mut self, wrapped_message: EventEnvelope) {
        let mut total;

        let blank = self.new_vec_envelope();

        {
            let ll = {
                self.messages.last()
            };

            let (mut fst, snd) = match ll {
                None =>
                    (vec![wrapped_message], blank),
                Some(most_recent) =>
                    if wrapped_message.timestamp >= most_recent.timestamp {
                        (vec![wrapped_message], self.messages.to_vec())
                    } else {
                        (vec![wrapped_message], blank)
                    }
            };

            fst.extend(snd);

            total = fst;
        }

        //let (f, s) = total;

        //self.messages = f;
    }

    fn new_vec_envelope (&self) -> Vec<EventEnvelope> {
        let blank: Vec<EventEnvelope> = Vec::new();
        blank
    }

    fn most_recent_message (&self) -> Option<&EventEnvelope> {
        self.messages.last()
    }
}

#[derive(Clone)]
pub struct EventEnvelope {
    contents: Message,
    timestamp: i32
}

#[derive(Clone)]
enum Message {
    RegularMessage {text: String}
}
