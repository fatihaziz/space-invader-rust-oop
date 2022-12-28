use std::io::Read;
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    let server = Server {
        connected_clients: Arc::new(Mutex::new(Vec::new())),
    };
    let cc = server.connected_clients.clone();

    thread::spawn(move || {
        let listener = TcpListener::bind("127.0.0.1:25565").unwrap();

        // For each new connection start a new thread
        for stream in listener.incoming() {
            let stream = Arc::new(Mutex::new(stream.unwrap()));
            let client = Client {
                stream: stream.clone(),
            };

            cc.lock().unwrap().push(client);

            thread::spawn(move || {
                // TODO: Add client to the connected_clients Vec
                let mut buffer = [0; 1024];
                loop {
                    stream.lock().unwrap().read(&mut buffer).unwrap();
                    println!(
                        "{}",
                        String::from_utf8(Vec::from(&buffer[..]))
                            .unwrap()
                            .trim_end_matches(char::from(0))
                    );
                }
            });
        }
    });

    loop {
        thread::sleep(Duration::from_secs(10));
        // let vec = server.lock().unwrap().connected_clients.lock().unwrap().iter();
        for client in server.connected_clients.lock().unwrap().iter() {
            println!("{:?}", client.stream.lock().unwrap().peer_addr().unwrap())
        }
    }
}

#[derive(Debug)]
struct Server {
    connected_clients: Arc<Mutex<Vec<Client>>>,
}

#[derive(Debug)]
struct Client {
    stream: Arc<Mutex<TcpStream>>,
}
