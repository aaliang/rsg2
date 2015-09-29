extern crate rsg2;

use rsg2::rsg2::{StreamMap};

fn main () {
    let rsm = &mut StreamMap::new();

    rsm.add_channel("topic1");

    rsm.add_channel("topic2");
}
