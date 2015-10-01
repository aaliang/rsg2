extern crate time;

use std::collections::HashMap;

enum ChanInstanceState <'a> {
    ExistingChannel,
    NewChannel (Channel <'a>) 
}

/// A map of Channels
pub struct StreamMap <'a> {
    channels: HashMap<String, Channel<'a>>,
    message_master: Vec<EventEnvelope>
}

impl <'a> StreamMap <'a> {
    pub fn new () -> StreamMap <'a> {
        StreamMap {
            channels: HashMap::new(),
            message_master: Vec::new()
        }
    }

    /// Adds a new Channel to the map
    ///
    pub fn add_channel (&self, topic: String) -> Channel {
        Channel::new(topic)
    }

    /// Adds ONE message to multiple channels
    pub fn append_message_to_channels (&'a mut self, mesg: EventEnvelope, chan_list: Vec<Channel>) {
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
                    //add the chan to the channels hashmap if it is new
                    self.channels.insert(chan.topic, n_chan);
                    ()
                }
                ChanInstanceState::ExistingChannel => ()
           };
        }
    }

    pub fn delete_message_from_master(&mut self) {
        self.message_master.clear();
    }
}

#[derive(Clone)]
pub struct Channel <'a> {
    topic: String,
    ttl: i32,
    messages: Vec<&'a EventEnvelope>
}

impl <'a> Channel <'a> {

    pub fn new (channel_name: String) -> Channel <'a> {
        Channel {
            topic: channel_name,
            ttl: 30,
            messages: Vec::new()
        }
    }

    pub fn append (&mut self, wrapped_message: &'a EventEnvelope) {
        self.messages.insert(0, wrapped_message);
    }

    /* BROKEN by the changes with message_master
    pub fn add_message (&mut self, wrapped_message: EventEnvelope) {
        let mut total;

        let blank = self.new_vec_envelope();

        {
            let (mut fst, snd) = match self.most_recent_message() {
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
    }

    pub fn most_recent_message (&self) {
        self.messages.last()
    }

    */

    /// clears messages
    pub fn truncate (&mut self) {
        self.messages.clear();
    }

    fn new_vec_envelope (&self) -> Vec<EventEnvelope> {
        let blank: Vec<EventEnvelope> = Vec::new();
        blank
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
