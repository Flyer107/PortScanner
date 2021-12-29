// INPUT : IPv4 address, Port Range
// FUNCTION : Enter an IP address and a port range 
//            where the program will then attempt to
//            find open ports on the given computer 
//            by connecting to each of them. On any 
//            successful connection ports, mark the 
//            port as open.
// OUTPUT : Status of port (open/closed)
use std::net::{TcpStream, SocketAddr};
use std::collections::HashMap;
use std::io;
use std::env;
use std::time::Duration;

//extern crate pnet;
//use pnet::datalink::{self, NetworkInterface};
//use pnet::datalink::Channel::Ethernet;

// Main fn to receive ip addr, start port, and end port
fn main() {
  let mut args: Vec<String> = env::args().collect();
  println!("Argument(s) Length: {}", args.len());

  if args.len() != 4 {
    let mut input = String::new();
    println!("Enter IP(v4): ");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    args.push(input.clone());
    input.clear();
    
    println!("Enter port start: ");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    args.push(input.clone());
    input.clear();
    
    println!("Enter port end: ");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    args.push(input.clone());
    input.clear();
  }
  
  println!("{:?}", args);
  
  
  let start_port: u16 = args[2].trim().parse::<u16>().unwrap();
  let end_port: u16 = args[3].trim().parse::<u16>().unwrap();

  let ports_open = scan_range(args[1].trim().to_string().clone(), start_port, end_port);
  
  let port_desc: HashMap<u16, &str> = [
    (20, "ftp"),
    (21, "ftp"),
    (22, "ssh"),
    (23, "telnet"),
    (50, "IPSec"),
    (51, "IPSec"),
    (53, "DNS"),
    (443, "HTTPS"),
  ].iter().cloned().collect();
  //println!("{}",ports_open.len());

  for key in ports_open.iter() {
    println!("{} is open", key);
    
    // if port_desc.contains_key(&key) {
    //   println!("{} is open: {:?}", key, port_desc.get_key_value(&key));
    // }
  }
  
}

// Scan within user specified range
fn scan_range(addr: std::string::String, start_port: u16, end_port: u16 ) -> Vec<u16> {
  let mut open_ports = Vec::new();
  for curr_port in start_port..end_port {
    let server_string = addr.clone() + ":" + curr_port.to_string().as_str().trim();
    let server: SocketAddr = server_string.parse().expect("Unable to parse socket address");
    if let Ok(_) = TcpStream::connect_timeout(&server, Duration::new(0, 500000)) {
      //println!("Connected to {}", server_string);
      open_ports.push(curr_port);
    } else {
      //println!("Couldn't connect to {}", server_string);
    }
  }
  open_ports
  // print from open_ports vector
}

// Scan within user specified range
fn scan_range_icmp(addr: std::string::String, start_port: u16, end_port: u16 ) -> Vec<u16> {
  let mut open_ports = Vec::new();
  for curr_port in start_port..end_port {
    let server_string = addr.clone() + ":" + curr_port.to_string().as_str().trim();
    let server: SocketAddr = server_string.parse().expect("Unable to parse socket address");
    if let Ok(_) = TcpStream::connect_timeout(&server, Duration::new(0, 500000)) {
      println!("Connected to {}", server_string);
      open_ports.push(curr_port);
    } else {
      println!("Couldn't connect to {}", server_string);
    }
  }
  open_ports
  // print from open_ports vector
}

//172.217.12.228:443
//https://subinsb.com/default-device-ttl-values/
//https://docs.rs/ttl_cache/0.5.1/ttl_cache/