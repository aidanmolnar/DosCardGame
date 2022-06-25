use std::thread;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};
use std::io;

use::bevy::prelude::*;


fn main() {

    let listener = TcpListener::bind("0.0.0.0:3333").unwrap();
    //listener.set_nonblocking(true).expect("Cannot set non-blocking");

    App::new()
        .insert_resource(listener)
        .add_plugins(MinimalPlugins)
        .add_system(listen_for_connections)
        .run()
}

fn listen_for_connections(listener: ResMut<TcpListener>) {
    // accept connections and process them, spawning a new thread for each one
    println!("Server listening on port 3333");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                thread::spawn(move|| {
                    // connection succeeded
                    handle_client(stream)
                });
            }
            // Err (ref e) if e.kind() == io::ErrorKind::WouldBlock => {       
            //     continue
            // }
            Err(e) => {
                println!("Error: {}", e);
                /* connection failed */
            }
        }
    }
    // close the socket server
    //drop(listener);
}

fn handle_client(mut stream: TcpStream) {
    let mut data = [0; 50]; // using 50 byte buffer
    while match stream.read(&mut data) {
        Ok(size) => {
            // echo everything!
            stream.write_all(&data[0..size]).unwrap();
            true
        },
        // TODO
        // Err (ref e) if e.kind() == io::ErrorKind::WouldBlock => {       
        //     continue
        // }
        Err(_) => {
            println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}
}
