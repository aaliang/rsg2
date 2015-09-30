extern crate time;

use std::collections::HashMap;

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
    pub fn add_channel (&mut self, topic: String) -> Channel {
        Channel::new(topic)
    }

    /// Adds ONE message to multiple channels
    pub fn add_message_to_channels (&'a mut self, mesg: EventEnvelope, chan_list: Vec<Channel>) {
        self.message_master.push(mesg);
        let message_ref = &self.message_master.last().unwrap();

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

            channel.append(message_ref);



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
