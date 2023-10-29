use winput::message_loop;
use winput::{Action, Vk};

use rust_socketio::{Payload, Socket, SocketBuilder};

fn main() {
    let receiver = message_loop::start().unwrap();

    let callback = |payload: Payload, _socket: Socket| match payload {
        Payload::String(str) => println!("{}", str[1..str.len() - 1].to_string()),
        Payload::Binary(bin_data) => println!("{:?}", bin_data),
    };
    

    let _socket = SocketBuilder::new("http://localhost:3000")
        .on("message", callback)
        .on("error", |err, _| eprintln!("Error: {:#?}", err))
        .connect()
        .expect("Connection failed");

    loop {
        match receiver.next_event() {
            message_loop::Event::Keyboard {
                vk,
                action: Action::Press,
                ..
            } => {
                if vk == Vk::Escape {
                    break;
                } else {
                    println!("{:?} was pressed!", vk);
                    let key_name = format!("{:?}", vk);
                    _socket
                        .emit("message", key_name)
                        .expect("Server unreachable");
                }
            }
            _ => (),
        }
    }
}
