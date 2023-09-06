use std::io::prelude::*;
use std::io::BufReader;
use std::net::{TcpListener, TcpStream};

fn handle_client(stream: TcpStream) -> std::io::Result<()> {
    let mut buffer = BufReader::new(&stream);

    let mut line = String::new();
    let len = buffer.read_line(&mut line)?;
    println!("First line is {len} bytes long");

    println!("Line is: {line}");

    Ok(())
}

fn main() -> std::io::Result<()> {
    if let Ok(mut stream) = TcpStream::connect("127.0.0.1:34070") {
        stream.write(b"Teste!!!")?;
        stream.flush()?;

        return Ok(());
    }

    let listener = TcpListener::bind("127.0.0.1:34070")?;

    for stream in listener.incoming() {
        match stream {
            Err(_) => println!("Erro ao escutar!"),
            Ok(stream) => {
                println!(
                    "Conexão de {} para {}",
                    stream.peer_addr().unwrap(),
                    stream.local_addr().unwrap()
                );

                handle_client(stream)?;
            }
        }
    }

    Ok(())
}
