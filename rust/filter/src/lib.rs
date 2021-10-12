use std::collections::HashMap;
use std::slice::from_raw_parts_mut;

#[no_mangle]
pub extern "C" fn ip_add(x : *mut u8) -> bool{
    let mut map = HashMap::new();

    let mac : [u8; 6] = [0x90, 0xe2, 0xba, 0xb1, 0x2c, 0x62];
    map.insert(mac,true);

    unsafe{
        let addr = from_raw_parts_mut(x, 6);

        let boolean = map.get(addr);


        match boolean {
            Some(i) => *i,
            None => false,
        }
    }
}
