extern crate rsg2;

use rsg2::rsg2::{StreamMap,StandardMessage};

fn main () {
    let rsm:StreamMap<StandardMessage> = StreamMap::new();

    let chan_1 = rsm.add_channel("topic1".to_string());
    let chan_2 = rsm.add_channel("topic2".to_string());

//:rsm.add_message_to_channels();

    let mut my_vec: Vec<i32> = Vec::new();

    my_vec.push(1i32);
    my_vec.push(2i32);

    let b_vec = &mut my_vec;

    {
        let last = b_vec.last();
        let ulast = last.unwrap();
    }

    b_vec.push(3i32);

/*    match last {
        Some(&1i32) =>
            b_vec.push(3i32),
        _ =>
            b_vec.push(4i32)
    }
*/
    
}
