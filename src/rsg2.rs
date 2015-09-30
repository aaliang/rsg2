extern crate time;

use std::collections::HashMap;

/// A map of Channels
pub struct StreamMap {
    channels: HashMap<String, Channel >
}

impl StreamMap {
    pub fn new () -> StreamMap {
        StreamMap {
            channels: HashMap::new()
        }
    }

    /// Adds a new Channel to the map
    ///
    pub fn add_channel (&mut self, topic: String) -> Channel {
        Channel::new(topic)
    }

    /// Adds ONE message to multiple channels
    pub fn add_message_to_channels (&mut self, mesg: EventEnvelope, chan_list: Vec<Channel>) {

        for chan in chan_list { 

            //cheap hack. need a reference to preserve the lifetime of a new 
            //Channel in the None arm. this is so i don't need lifetime annotations
            let mut temp;

            let (channel, exists) = match self.channels.get_mut(&chan.topic) {
                Some (a) => (a, true),
                None => {
                    temp = Channel::new(chan.topic);
                    (&mut temp, false)
                }
            };

            



            //let chan_handle = chan_opt.unwrap().insert(chan.topic.clone(), Channel::new(chan.topic));

            /*let chan_handle = match chan_opt {
                Some(ref a) => *a,
                None => {
                    let t = chan.topic.clone();
                    let mut new_chan = Channel::new(chan.topic);
                    &mut self.channels.insert(t, new_chan).unwrap()
                }
            };*/
        }
    }
}

pub struct Channel {
    topic: String,
    ttl: i32,
    messages: Vec<EventEnvelope>
}

impl Channel {

    pub fn new <'a> (channel_name: String) -> Channel {
        Channel {
            topic: channel_name,
            ttl: 30,
            messages: Vec::new()
        }
    }

    pub fn get_mut_ref <'a> (&'a self) -> &'a Channel {
        self
    }

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
