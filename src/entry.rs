extern crate rsg2;

use rsg2::rsg2::{StreamMap};

fn main () {
    let rsm = &mut StreamMap::new();

    rsm.add_channel("topic1");
    rsm.add_channel("topic2");

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
