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

    fn add_message (self, wrapped_message: EventEnvelope) {
        
        let mut messages = self.messages;

        match (messages.last()) {
            None => messages.push(wrapped_message),
            _ => messages.push(wrapped_message)
        }
        //match (messages.last()) {
        //    None => 
        //
        //        messages.push(wrapped_message),
        //    Some (a) =>
        //        if (wrapped_message.timestamp >= a.timestamp) {
        //            messages.push(wrapped_message)
        //        }
        //        else {
        //            messages.push(wrapped_message)
        //        }
       // }

    }
}

pub struct EventEnvelope {
    contents: Message,
    timestamp: i32
}

enum Message {
    RegularMessage {text: String}
}
