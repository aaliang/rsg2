extern crate time;

use std::collections::HashMap;

enum ChanInstanceState <'a, T: 'a> {
    ExistingChannel,
    NewChannel (Channel <'a, T>) 
}

pub struct StandardMessage {
    text: String
}

/// A map of Channels
pub struct StreamMap <'a, T: 'a> {
    channels: HashMap<String, Channel<'a, T>>,
    message_master: Vec<EventEnvelope<T>>
}

impl <'a, T: 'a> StreamMap <'a, T> {

    /// Instantiates a generic StreamMap
    pub fn new () -> StreamMap <'a, T> {
        let hm:HashMap<String, Channel<T>> = HashMap::new();
        let mm:Vec<EventEnvelope<T>> = Vec::new();
        StreamMap {
            channels: hm,
            message_master: mm
        }
    }

    /// Adds a new Channel to the map
    ///
    pub fn add_channel (&self, topic: String) -> Channel <T> {
        Channel::new(topic)
    }

    /// Adds ONE message to multiple channels
    pub fn append_message_to_channels (&'a mut self, mesg: EventEnvelope<T> , chan_list: Vec<Channel<T>>) {
        self.message_master.push(mesg);
        let message_ref = &self.message_master.last().unwrap();

        for chan in chan_list { 
            let instance_state = {
                match self.channels.get_mut(&chan.topic) {
                    Some (a) => {
                        a.append(message_ref);
                        ChanInstanceState::ExistingChannel
                    },
                    None => {
                        let mut new_chan = Channel::new(chan.topic.clone());
                        new_chan.append(message_ref);
                        ChanInstanceState::NewChannel(new_chan)
                    }
                }
            };
            match instance_state {
                ChanInstanceState::NewChannel(n_chan) => {
                    self.channels.insert(chan.topic, n_chan);
                },
                ChanInstanceState::ExistingChannel => ()
           };
        }
    }

    pub fn delete_message_from_master(&mut self) {
        self.message_master.clear();
    }
}

#[derive(Clone)]
pub struct Channel <'a, T: 'a> {
    topic: String,
    ttl: i32,
    messages: Vec<&'a EventEnvelope<T>>
}

impl <'a, T> Channel <'a, T> {

    pub fn new (channel_name: String) -> Channel <'a, T> {
        Channel {
            topic: channel_name,
            ttl: 30,
            messages: Vec::new()
        }
    }

    pub fn append (&mut self, wrapped_message: &'a EventEnvelope<T>) {
        self.messages.insert(0, wrapped_message);
    }

    pub fn add_to_correct_position (&mut self, wrapped_message: &'a EventEnvelope<T>) {

        let is_empty = self.messages.is_empty();

        if is_empty {
            self.messages.push(wrapped_message);
        }
        else {
            let is_appendable = {
                let most_recent = self.messages.last().unwrap();
                most_recent.timestamp <= wrapped_message.timestamp
            };
            if is_appendable {
                self.messages.push(wrapped_message);
            } else {
               //TODO: yeah 
            }
        }
    }

    /// clears messages
    pub fn truncate (&mut self) {
        self.messages.clear();
    }

    pub fn get_position_to_insert (&self) {
    }
}

#[derive(Clone)]
pub struct EventEnvelope <T> {
    contents: T,
    timestamp: u64
}

impl <T> EventEnvelope <T> {
    pub fn new_message(contents: T) -> EventEnvelope <T> {
        EventEnvelope {
            contents: contents,
            timestamp: time::precise_time_ns()
        }
    }
}

#[derive(Clone)]
enum Message {
    RegularMessage {text: String}
}
