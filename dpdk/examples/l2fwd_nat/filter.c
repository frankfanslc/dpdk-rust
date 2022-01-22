use std::collections::HashMap;
use std::slice::from_raw_parts_mut;
use std::thread;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};

fn parse_query(m: &str) -> [u8; 6]{
    let a = "90:e2:ba:b1:2c:62";
    let vec: Vec<&str> = a.split(':').collect();
    let mut mac: [u8;6] = Default::default();
    // let mut mac: [u8; 6] = Default::default();
    // for i in b.iter(){
    //     mac[0] = i.parse::<u8>().unwrap();
    // }
    for i in 0..6 {
        mac[i] = hex::decode(&vec[i]).unwrap().pop().unwrap();
    }
    mac
}



fn start_server(listener: &TcpListener, map: Arc<Mutex<HashMap<[u8;6], bool>>>){
    let stream = listener.accept().expect("Error. failed to accept");

    let mut buf =[0; 1024];
    let mut tcp_stream: TcpStream = stream.0;
    let mut map_cont = map;
    loop{

        tcp_stream.write("\nadd or delete\n".as_bytes()).expect("Error. failed to send");
        let  mut sz = tcp_stream.read(&mut buf).expect("Error. failed to recieve");
        //if sz == 0 {
        //    return;
        //}
        let word = String::from_utf8_lossy(&buf[..(sz - 2)]);
        if "add" == word {
            tcp_stream.write("---Insert Mode---\n".as_bytes()).expect("Error. failed to send");
            sz = tcp_stream.read(&mut buf).expect("Error. failed to recieve");
            let add_word = String::from_utf8_lossy(&buf[..(sz - 2)]);
            tcp_stream.write(&buf[..(sz - 2)]).expect("Error. failed to send");
            tcp_stream.write(" is added\n".as_bytes()).expect("Error. failed to send");

            let mac = parse_query(&add_word);
            //let add_key = add_word.parse::<u64>().unwrap();
            map_cont = add_map(map_cont, mac);

        }else if "delete" == word {
            tcp_stream.write("---Remove Mode---\n".as_bytes()).expect("Error. failed to send");
            sz = tcp_stream.read(&mut buf).expect("Error. failed to recieve");
            let del_word = String::from_utf8_lossy(&buf[..(sz - 2)]);
            tcp_stream.write(&buf[..(sz - 2)]).expect("Error. failed to send");
            tcp_stream.write(" is deleted\n".as_bytes()).expect("Error. failed to send");

            let mac = parse_query(&del_word);
            //let del_key = del_word.parse::<u64>().unwrap();
            map_cont = del_map(map_cont, mac);
        }else {
            tcp_stream.write("try again....\n".as_bytes()).expect("Error. failed to send");
            continue;
        }
    }
}

fn tcp_listen(map: Arc<Mutex<HashMap<[u8;6], bool>>>) {

    println!("init listener");
    let listener = TcpListener::bind("127.0.0.1:1935").expect("Error. failed to bind.");

    //loop{
        println!("Open 127.0.0.1:1935, wait connect...");
        start_server(&listener , map);
    //}
}


#[no_mangle]
pub extern "C" fn gen_map() -> *const () {
    let map_org: Arc<Mutex<HashMap<[u8;6], bool>>> = Arc::new(Mutex::new(HashMap::new()));

    {
        let mut map = map_org.lock().unwrap();
        //map.insert(3, false);
    }
    let map_clone = map_org.clone();

    thread::spawn(move ||{
        println!("At thread add 5");
        //let map = add_map(map_clone, 5);
        tcp_listen(map_clone);
        loop{
            thread::sleep(Duration::from_secs(3));
            println!("-----thread th_add active----");
        }
    });
    Arc::into_raw(map_org) as *const ()
}

#[no_mangle]
pub extern "C" fn read_map(map: *const (), mac: *mut u8) -> *const (){

    let a = map as *const Mutex<HashMap<[u8;6], bool>>;
    let x = unsafe{Arc::from_raw(a)};
    let mac_addr = from_raw_parts_mut(x, 6);
    //let mac : [u8; 6] = [0x90, 0xe2, 0xba, 0xb1, 0x2c, 0x62];
    {
        let y = x.lock().unwrap();

        {
            for (key, val) in y.iter() {
                for i in 0..5{
                    println!("key: {} val: {}", key[i], val);
                    println!("key: {}", mac[i]);
                }
            }
        }

        let boolean = y.get(mac_addr);

        match boolean {
            Some(i) => {
                println!("there is {}, value:{}", mac_addr, *i);
            },
            None => {
                println!("there is not mac_address");
            },
        }
    }
    Arc::into_raw(x) as *const ()

}

fn add_map(map: Arc<Mutex<HashMap<[u8;6], bool>>>, num: [u8;6]) -> Arc<Mutex<HashMap<[u8;6], bool>>>{

    //let a = map as *const Mutex<HashMap<i32, bool>>;
    //let x = unsafe{Arc::from_raw(a)};
    {
        let mut y = map.lock().unwrap();
        let z = y.insert(num, true);
        match z {
            Some(_) => println!("update!!\n{:?} : true", num ),
            None => println!("new one!!"),
        }
    }
    map
}

fn del_map(map: Arc<Mutex<HashMap<[u8;6], bool>>>, num: [u8;6]) -> Arc<Mutex<HashMap<[u8;6], bool>>>{

    //let a = map as *const Mutex<HashMap<i32, bool>>;
    //let x = unsafe{Arc::from_raw(a)};
    {
        let mut y = map.lock().unwrap();
        let z = y.remove(&num);

        match z {
            Some(i) => println!("{:?} : {} is removed!!", num, i),
            None => println!("was not removed"),
        }
    }
    map
}

#[no_mangle]
pub extern "C" fn check_map(map: *const ()) -> *const (){

    let a = map as *const Mutex<HashMap<[u8;6], bool>>;
    let x = unsafe{Arc::from_raw(a)};
    {
        let y = x.lock().unwrap();

        for (key, val) in y.iter() {
            for i in 0..6{
                println!("key: {} val: {}", key[i], val);
            }
        }
    }
    Arc::into_raw(x) as *const ()
}
