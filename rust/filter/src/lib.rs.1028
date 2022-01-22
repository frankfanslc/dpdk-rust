use std::collections::HashMap;
use std::slice::from_raw_parts_mut;
use std::thread;
use std::sync::{Arc, Mutex};
//use std::time::Duration;
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};

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

fn parse_query(m: &str) -> [u8; 6]{
    //let a = "90:e2:ba:b1:2c:62";
    let vec: Vec<&str> = m.split(':').collect();
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

        tcp_stream.write("\nEnter a command (add or delete)\n".as_bytes()).expect("Error. failed to send");
        let  mut sz = tcp_stream.read(&mut buf).expect("Error. failed to recieve");
        //if sz == 0 {
        //    return;
        //}
        let word = String::from_utf8_lossy(&buf[..(sz - 2)]);
        if "add" == word {
            tcp_stream.write("==========Insert Mode==========\nMac address => ".as_bytes()).expect("Error. failed to send");
            sz = tcp_stream.read(&mut buf).expect("Error. failed to recieve");
            let add_word = String::from_utf8_lossy(&buf[..(sz - 2)]);
            tcp_stream.write(&buf[..(sz - 2)]).expect("Error. failed to send");
            tcp_stream.write(" <= [added]\n".as_bytes()).expect("Error. failed to send");
            tcp_stream.write("=======Insert Mode closed=======\n".as_bytes()).expect("Error. failed to send");

            let mac = parse_query(&add_word);
            //let add_key = add_word.parse::<u64>().unwrap();
            map_cont = add_map(map_cont, mac);

        }else if "delete" == word {
            tcp_stream.write("==========Remove Mode==========\nMac address => ".as_bytes()).expect("Error. failed to send");
            sz = tcp_stream.read(&mut buf).expect("Error. failed to recieve");
            let del_word = String::from_utf8_lossy(&buf[..(sz - 2)]);
            tcp_stream.write(&buf[..(sz - 2)]).expect("Error. failed to send");
            tcp_stream.write(" <= [deleted]\n".as_bytes()).expect("Error. failed to send");
            tcp_stream.write("=======Remove Mode closed=======\n".as_bytes()).expect("Error. failed to send");

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
        //let mut map = map_org.lock().unwrap();
        //map.insert(3, false);
    }
    let map_clone = map_org.clone();

    thread::spawn(move ||{
        //println!("At thread add 5");
        //let map = add_map(map_clone, 5);
        tcp_listen(map_clone);
        //loop{
        //    thread::sleep(Duration::from_secs(3));
        //    println!("-----thread th_add active----");
        //}
    });
    Arc::into_raw(map_org) as *const ()
}

#[repr(C)]
pub struct Mapbool {
    x: bool,
    y: *const (),
}

#[no_mangle]
pub extern "C" fn read_map(map: *const (), mac: *mut u8) -> Mapbool{

    let a = map as *const Mutex<HashMap<[u8;6], bool>>;
    let x = unsafe{Arc::from_raw(a)};
    let mac_addr = unsafe{from_raw_parts_mut(mac, 6)};
    //let mac : [u8; 6] = [0x90, 0xe2, 0xba, 0xb1, 0x2c, 0x62];
    let ret: bool;
    {
        let y = x.lock().unwrap();

        //{
        //    for (key, _) in y.iter() {
        //        for i in 0..5{
        //            print!("{}:", key[i]);
        //            //println!("key: {}", mac_addr[i]);
        //        }
        //    }
        //}
    
        let boolean = y.get(mac_addr);

        match boolean {
            Some(i) => {
                //println!("there is {:?}, value:{}", mac_addr, *i);
                for (key, value) in y.iter() {
                    print!("[From]");
                    for j in 0..6{
                        print!("{:X}", key[j]);
                        if j != 5 {print!(":");}
                        else {print!(" ===> [{}]\n", value);}
                    }
                }   
                ret = *i;
            },
            None => {
                print!("[From]");
                for j in 0..6{
                    print!("{:X}", mac_addr[j]);
                    if j != 5 {print!(":");}
                    else {print!(" ===> [Undefined]\n");}
                }

                //println!("filter is not defined");
                ret = false;
            },
        }
    }
    Mapbool{ x: ret, y: Arc::into_raw(x) as *const () }
}

fn add_map(map: Arc<Mutex<HashMap<[u8;6], bool>>>, num: [u8;6]) -> Arc<Mutex<HashMap<[u8;6], bool>>>{

    //let a = map as *const Mutex<HashMap<i32, bool>>;
    //let x = unsafe{Arc::from_raw(a)};
    {
        let mut y = map.lock().unwrap();
        let z = y.insert(num, true);
        match z {
            Some(_) => {
                println!("\n=======Map Updated=======");
                for i in 0..6{
                    print!("{:X}", num[i]);
                    if i != 5 {print!(":");}
                }
                println!(" = true");
                println!("=========================\n");
            },
            None => {
                println!("\n=========New one=========");
                for i in 0..6{
                    print!("{:X}", num[i]);
                    if i != 5 {print!(":");}
                }
                println!(" = true");
                println!("=========================\n");
            },
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
            Some(j) => {
                println!("\n=======Removed one=======");
                for i in 0..6{
                    print!("{:X}", num[i]);
                    if i != 5 {print!(":");}
                }
                println!(" = {}", j);
                println!("=========================\n");
            },
            None => println!("Error.. it was not removed"),
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
