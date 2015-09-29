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

    fn add_message (&mut self, wrapped_message: EventEnvelope) {

        let mut total;

        {
            let last = self.last_message();

            let mut _mid:Vec<EventEnvelope> = Vec::new();
            let mut _end:Vec<EventEnvelope> = Vec::new();

            let (mut begin, mut mid, mut end) = match last {
                None => 
                    (vec![wrapped_message], _mid, _end),
                Some(a) =>
                    (vec![wrapped_message], _mid, _end)
            };

            begin.extend(mid);
            begin.extend(end);

            total = begin;
        }

        self.messages = total;
    }

    fn last_message (&mut self) -> Option<&EventEnvelope> {
        self.messages.last()
    }
}

fn get_last_message (channel: &Channel) -> Option <&EventEnvelope> {
    channel.messages.last()
}

pub struct EventEnvelope {
    contents: Message,
    timestamp: i32
}

enum Message {
    RegularMessage {text: String}
}
