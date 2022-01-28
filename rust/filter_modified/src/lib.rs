#![feature(box_patterns)]
use prusti_contracts::*;
//use std::collections::HashMap;
//use std::slice::from_raw_parts_mut;
use std::thread;
use std::sync::{Arc, Mutex};
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::mem;
use std::convert::TryInto;

pub struct ListMap {
    head: Link,
    head_m: Link
}

enum Link {
    NULL,
    Data(Box<Node>)
}

struct Node {
    elem: (u64, u64),
    next: Link,
}

#[derive(Clone, Copy)]
pub enum TrustedOption {
    Some(u64),
    None,
}

impl TrustedOption {

    #[pure]
    pub fn eql(&self, value: TrustedOption) -> bool {
        match self {
            TrustedOption::Some(i) => {
                match value {
                    TrustedOption::Some(j) => *i == j,
                    TrustedOption::None => false,
                }
            }
            TrustedOption::None => {
                match value {
                    TrustedOption::Some(_) => false,
                    TrustedOption::None => true,
                }
            }
        }
    }
}

#[trusted]
#[requires(src.is_empty())]
#[ensures(dest.is_empty())]
fn replace(dest: &mut Link, src: Link) -> Link {
    mem::replace(dest, src)
}

impl ListMap {
    #[ensures(forall(|i: u64| (result.check(i) == false)))]
    pub fn new() -> Self {
        ListMap {
            head: Link::NULL,
            head_m: Link::NULL
        }
    }

    #[ensures(self.get(elem_k).eql(TrustedOption::Some(elem_v)))]
    #[ensures(self.get_m(elem_v).eql(TrustedOption::Some(elem_k)))]
    pub fn insert(&mut self, elem_k: u64, elem_v : u64) {
        let new_node = Box::new(Node{
            elem: (elem_k, elem_v),
            next: replace(&mut self.head, Link::NULL),
        });
        self.head = Link::Data(new_node);

        let new_node_m = Box::new(Node{
            elem: (elem_v, elem_k),
            next: replace(&mut self.head_m, Link::NULL),
        });
        self.head_m = Link::Data(new_node_m);
    }

    #[ensures(self.check(elem_k) == false)]
    #[ensures(self.check_m(elem_v) == false)]
    pub fn remove(&mut self, elem_k: u64, elem_v: u64) {
        match replace(&mut self.head, Link::NULL) {
            Link::NULL => (),
            Link::Data(box mut node) => {
                node.next.remove(elem_k);
                if node.elem.0 == elem_k {
                    self.head = node.next;
                } else {
                    self.head = Link::Data(Box::new(node));
                }
            }
        }

        match replace(&mut self.head_m, Link::NULL) {
            Link::NULL => (),
            Link::Data(box mut node_m) => {
                node_m.next.remove(elem_v);
                if node_m.elem.0 == elem_v {
                    self.head_m = node_m.next;
                } else {
                    self.head_m = Link::Data(Box::new(node_m));
                }
            }
        }
    }

    #[pure]
    pub fn get(&self, elem_k: u64) -> TrustedOption {
        self.head.get(elem_k)
    }

    #[pure]
    pub fn get_m(&self, elem_v: u64) -> TrustedOption {
        self.head_m.get(elem_v)
    }

    #[pure]
    pub fn check(&self, elem: u64) -> bool {
        self.head.check(elem)
    }

    #[pure]
    pub fn check_m(&self, elem: u64) -> bool {
        self.head_m.check(elem)
    }

    pub fn search(&self) {
        self.head.search()
    }
}

impl Link {
    
    #[ensures(self.check(elem) == false)]
    fn remove(&mut self, elem: u64) {
        match replace(self, Link::NULL) {
            Link::NULL => (),
            Link::Data(box mut node) => {
                if node.elem.0 == elem {
                    *self = node.next;
                    self.remove(elem);
                } else {
                    node.next.remove(elem);
                    *self = Link::Data(Box::new(node)); 
                }
            }
        }
    }

    #[pure]
    fn get(&self, elem: u64) -> TrustedOption {
        match self {
            Link::NULL => TrustedOption::None,
            Link::Data(box node) => {
                if node.elem.0 == elem {
                    TrustedOption::Some(node.elem.1)
                } else {
                    node.next.get(elem)
                }
            }
        }
    }

    #[pure]
    fn check(&self, elem: u64) -> bool {
        match self {
            Link::NULL => false,
            Link::Data(box node) => {
                if node.elem.0 == elem {
                    true
                } else {
                    node.next.check(elem)
                }
            }
        }
    }

    fn search(&self) {
        match self {
            Link::NULL => (),
            Link::Data(box node) => {
                println!("\n[Search] src = {}.{}.{}.{}.{}", ((node.elem.0 >> 16) & ((2 << 7) -1)), ((node.elem.0 >> 24) & ((2 << 7) -1)), ((node.elem.0 >> 32) & ((2 << 7) -1)), ((node.elem.0 >> 40) & ((2 << 7) -1)), (node.elem.0 & ((2 << 15) -1)));
                println!("\n[Search] dst = {}.{}.{}.{}.{}", ((node.elem.1 >> 16) & ((2 << 7) -1)), ((node.elem.1 >> 24) & ((2 << 7) -1)), ((node.elem.1 >> 32) & ((2 << 7) -1)), ((node.elem.1 >> 40) & ((2 << 7) -1)), (node.elem.1 & ((2 << 15) -1)));
                node.next.search()
            }
        }
    }

    #[pure]
    #[allow(unused)]
    fn is_empty(&self) -> bool {
        match self {
            Link::NULL => true,
            Link::Data(box node) => false,
        }
    }
}

fn parse_query(m: &str) -> u64{
    //let a = "90:e2:ba:b1:2c:62";
    let vec: Vec<&str> = m.split('.').collect();
    let mut mac: [u64;5] = Default::default();
    // let mut mac: [u8; 6] = Default::default();
    // for i in b.iter(){
    //     mac[0] = i.parse::<u8>().unwrap();
    // }
    for i in 0..5 {
        mac[i] = vec[i].parse().unwrap();
    }
    //let ret: u64 = (mac[0] << 40) + (mac[1] << 32) + (mac[2] << 24) + (mac[3] << 16) + mac[4];
    let ret: u64 = (mac[3] << 40) + (mac[2] << 32) + (mac[1] << 24) + (mac[0] << 16) + mac[4];
    ret
}



fn start_server(listener: &TcpListener, map: Arc<Mutex<ListMap>>){
    let stream = listener.accept().expect("Error. failed to accept");

    let mut buf_src =[0; 1024];
    let mut buf_dst =[0; 1024];
    let mut tcp_stream: TcpStream = stream.0;
    let mut map_cont = map;
    loop{

        tcp_stream.write("\nEnter a command (add or delete)\n".as_bytes()).expect("Error. failed to send");
        let  mut sz = tcp_stream.read(&mut buf_src).expect("Error. failed to recieve");
        //if sz == 0 {
        //    return;
        //}
        let word = String::from_utf8_lossy(&buf_src[..(sz - 2)]);
        if "add" == word {
            tcp_stream.write("==========Insert Mode==========\nSrc IP_Port => ".as_bytes()).expect("Error. failed to send");
            sz = tcp_stream.read(&mut buf_src).expect("Error. failed to recieve");
            let src_add_word = String::from_utf8_lossy(&buf_src[..(sz - 2)]);

            tcp_stream.write("Dst IP_Port => ".as_bytes()).expect("Error. failed to send");
            sz = tcp_stream.read(&mut buf_dst).expect("Error. failed to recieve");
            let dst_add_word = String::from_utf8_lossy(&buf_dst[..(sz - 2)]);

            tcp_stream.write(&buf_src[..(sz - 2)]).expect("Error. failed to send");
            tcp_stream.write(" <= [added]\n".as_bytes()).expect("Error. failed to send");
            tcp_stream.write("=======Insert Mode closed=======\n".as_bytes()).expect("Error. failed to send");

            let src_ip_port = parse_query(&src_add_word);
            //println!("\n\n\n\n\n\n\n\n\n[Rust] src_ip_port==={}\n\n\n\n\n\n\n\n", src_ip_port);
            let dst_ip_port = parse_query(&dst_add_word);
            //let add_key = add_word.parse::<u64>().unwrap();
            map_cont = add_map(map_cont, src_ip_port, dst_ip_port);

        }else if "delete" == word {
            tcp_stream.write("==========Remove Mode==========\nMac address => ".as_bytes()).expect("Error. failed to send");
            sz = tcp_stream.read(&mut buf_src).expect("Error. failed to recieve");
            let src_del_word = String::from_utf8_lossy(&buf_src[..(sz - 2)]);
            
            tcp_stream.write("==========Remove Mode==========\nMac address => ".as_bytes()).expect("Error. failed to send");
            sz = tcp_stream.read(&mut buf_dst).expect("Error. failed to recieve");
            let dst_del_word = String::from_utf8_lossy(&buf_dst[..(sz - 2)]);
            
            tcp_stream.write(&buf_src[..(sz - 2)]).expect("Error. failed to send");
            tcp_stream.write(" <= [deleted]\n".as_bytes()).expect("Error. failed to send");
            tcp_stream.write("=======Remove Mode closed=======\n".as_bytes()).expect("Error. failed to send");

            let src_ip_port = parse_query(&src_del_word);
            let dst_ip_port = parse_query(&dst_del_word);
            //let del_key = del_word.parse::<u64>().unwrap();
            map_cont = del_map(map_cont, src_ip_port, dst_ip_port);
        
        }else if "search" == word {
            map_cont = check_map(map_cont);
        }else {
            tcp_stream.write("try again....\n".as_bytes()).expect("Error. failed to send");
            continue;
        }
    }
}

fn tcp_listen(map: Arc<Mutex<ListMap>>) {

    //println!("init listener");
    let listener = TcpListener::bind("127.0.0.1:1935").expect("Error. failed to bind.");

    //loop{
        //println!("Open 127.0.0.1:1935, wait connect...");
        start_server(&listener , map);
    //}
}


#[no_mangle]
pub extern "C" fn gen_map() -> *const () {
    let map_org: Arc<Mutex<ListMap>> = Arc::new(Mutex::new(ListMap::new()));

    let map_clone = map_org.clone();

    thread::spawn(move ||{
        tcp_listen(map_clone);
    });
    Arc::into_raw(map_org) as *const ()
}

#[repr(C)]
pub struct Mapbool {
    ip: u32,
    port: u16,
    y: *const (),
    z: bool,
}


// fn calc_ip_port(boolean: bool, val: u64, p: *const()) -> Mapbool {
//     let ip_0: u8 = ((val >> 16) & ((2 << 7) - 1)).try_into().unwrap();
//     let ip_1: u8 = ((val >> 24) & ((2 << 7) - 1)).try_into().unwrap();
//     let ip_2: u8 = ((val >> 32) & ((2 << 7) - 1)).try_into().unwrap();
//     let ip_3: u8 = ((val >> 40) & ((2 << 7) - 1)).try_into().unwrap();
//     let port: u16 = (val & ((2 << 15) - 1)).try_into().unwrap();
//     Mapbool{ ip_0: ip_0, ip_1: ip_1, ip_2: ip_2, ip_3: ip_3, port: port, y: p, z: boolean }
// }

fn calc_ip_port(boolean: bool, val: u64, p: *const()) -> Mapbool {
    let ip: u32 = ((val >> 16) & ((2 << 31) - 1)).try_into().unwrap();
    let port: u16 = (val & ((2 << 15) - 1)).try_into().unwrap();
    Mapbool{ ip: ip, port: port, y: p, z: boolean }
}

#[no_mangle]
pub extern "C" fn read_map(map: *const (), src_ip_port: u64, t: i32) -> Mapbool{

    let a = map as *const Mutex<ListMap>;
    let x = unsafe{Arc::from_raw(a)};
    //let mac_addr = unsafe{from_raw_parts_mut(mac, 6)};

    let ret: u64;
    let boolean: bool;
    let dst_ip_port: TrustedOption;
    
    {
        let y = x.lock().unwrap();
        if t == 1 {
            dst_ip_port = y.get(src_ip_port);
        } else {
            dst_ip_port = y.get_m(src_ip_port);
        }

        match dst_ip_port {
            TrustedOption::Some(i) => {
                   
                ret = i;
                boolean = true;
            },
            TrustedOption::None => {

                ret = 0;
                boolean = false;
            },
        }
    }
    calc_ip_port(boolean, ret, Arc::into_raw(x) as *const ())
}

fn add_map(map: Arc<Mutex<ListMap>>, num: u64, num_m: u64) -> Arc<Mutex<ListMap>>{

    //let a = map as *const Mutex<HashMap<i32, bool>>;
    //let x = unsafe{Arc::from_raw(a)};
    {
        let mut y = map.lock().unwrap();
        let z = y.insert(num, num_m);
     
        println!("---- Added ----");
    }
    map
}

fn del_map(map: Arc<Mutex<ListMap>>, num: u64, num_m: u64) -> Arc<Mutex<ListMap>>{

    //let a = map as *const Mutex<HashMap<i32, bool>>;
    //let x = unsafe{Arc::from_raw(a)};
    {
        let mut y = map.lock().unwrap();
        let z = y.remove(num, num_m);

        println!("deleted!!!!");
    }
    map
}

fn check_map(map: Arc<Mutex<ListMap>>) -> Arc<Mutex<ListMap>>{

    {
        let y = map.lock().unwrap();
        //192.168.1.1.2240
        let val: u64 = 1106637752512;
        // match y.get(val) {
        //     TrustedOption::Some(i) => {
        //         println!("\n[Check] dst = {}.{}.{}.{}.{}", ((i >> 16) & ((2 << 7) -1)), ((i >> 24) & ((2 << 7) -1)), ((i >> 32) & ((2 << 7) -1)), ((i >> 40) & ((2 << 7) -1)), (i & ((2 << 7) -1)));
        //     },
        //     TrustedOption::None => {
        //         println!("THERE IS NOTHING!!");
        //     }
        // }
        y.search();
    }
    map
}
