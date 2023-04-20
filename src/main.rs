use clap::Parser;
use std::time::Duration;
use tokio::net::{windows::named_pipe::ClientOptions, TcpListener, TcpStream};
use tokio::{io, time};
use windows_sys::Win32::Foundation::ERROR_PIPE_BUSY;

#[derive(Parser, Debug, Clone)]
#[command(version = env!("CARGO_PKG_VERSION"))]
struct Args {
    host: String,
    port: String,
    pipe: String,
}

impl Args {
    fn address(&self) -> String {
        return format!("{}:{}", self.host, self.port);
    }
}

#[tokio::main]
async fn main() -> io::Result<()> {
    println!("Starting");
    let args = Args::parse();
    let listener = TcpListener::bind(args.address()).await?;
    println!("Listening on {}", args.address());
    loop {
        match listener.accept().await {
            Ok((socket, addr)) => {
                println!("new client: {:?}", addr);
                let pipe = args.pipe.clone();
                tokio::spawn(async move {
                    handle_client(socket, pipe).await.unwrap();
                });
            }
            Err(e) => println!("couldn't get client: {:?}", e),
        }
    }
}

async fn handle_client(mut stream: TcpStream, pipe: String) -> io::Result<()> {
    let mut client = loop {
        match ClientOptions::new().open(&pipe) {
            Ok(client) => break client,
            Err(e) if e.raw_os_error() == Some(ERROR_PIPE_BUSY as i32) => (),
            Err(e) => return Err(e),
        }
        println!("Pipe not open yet");
        time::sleep(Duration::from_millis(50)).await;
    };
    println!("Pipe connected");
    io::copy_bidirectional(&mut stream, &mut client).await?;
    println!("Finished handling client");
    Ok(())
}
