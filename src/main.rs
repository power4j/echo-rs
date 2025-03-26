use clap::Parser;
use std::net::{IpAddr, Ipv6Addr, SocketAddr};
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream, UdpSocket};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, default_value_t = 34567)]
    port: u16,

    #[clap(long, default_value_t = false)]
    ipv6: bool,

    #[clap(short, long, default_value_t = 1)]
    threads: usize,

    #[clap(short, long, default_value_t = 1)]
    min_response_len: usize,

    #[clap(short, long, default_value_t = false)]
    verbose: bool,
}

async fn handle_tcp_connection(
    mut stream: TcpStream,
    addr: SocketAddr,
    min_response_len: usize,
    verbose: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("[TCP] Connection from {} established.", addr);
    let mut buf = vec![0; 1024];
    let mut received_data = Vec::new();

    loop {
        let n = stream.read(&mut buf).await?;
        if n == 0 {
            println!("[TCP] Connection from {} closed.", addr);
            break;
        }

        received_data.extend_from_slice(&buf[..n]);

        if received_data.len() >= min_response_len {
            let sent_len = stream.write(&received_data).await?;
            if verbose {
                println!("[TCP] Sent {} bytes to {}", sent_len, addr);
            }
            received_data.clear();
        }
        if verbose {
            println!("[TCP] Received {} bytes from {}", n, addr);
        }
    }

    Ok(())
}

async fn handle_udp_packet(
    socket: Arc<UdpSocket>,
    buf: Vec<u8>,
    len: usize,
    addr: SocketAddr,
    min_response_len: usize,
    verbose: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    if len >= min_response_len {
        let sent_len = socket.send_to(&buf[..len], addr).await?;
        if verbose {
            println!("[UDP] Sent {} bytes to {}", sent_len, addr);
        }
    }
    if verbose {
        println!("[UDP] Received {} bytes from {}", len, addr);
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let addr = if args.ipv6 {
        SocketAddr::new(IpAddr::V6(Ipv6Addr::UNSPECIFIED), args.port)
    } else {
        SocketAddr::new(IpAddr::V4(std::net::Ipv4Addr::UNSPECIFIED), args.port)
    };

    let tcp_listener = TcpListener::bind(addr).await?;
    let udp_socket = Arc::new(UdpSocket::bind(addr).await?);

    println!(
        "Echo server listening on {}, {} threads, min response len: {}, verbose: {}",
        addr, args.threads, args.min_response_len, args.verbose
    );

    let udp_socket_clone = udp_socket.clone();
    let min_response_len_tcp = args.min_response_len;
    let min_response_len_udp = args.min_response_len;
    let verbose = args.verbose;

    tokio::spawn(async move {
        let mut buf = [0; 1024];
        loop {
            match udp_socket_clone.recv_from(&mut buf).await {
                Ok((len, addr)) => {
                    let udp_socket_clone_inner = udp_socket_clone.clone();
                    tokio::spawn(async move {
                        if let Err(e) = handle_udp_packet(
                            udp_socket_clone_inner,
                            buf.to_vec(),
                            len,
                            addr,
                            min_response_len_udp,
                            verbose,
                        )
                        .await
                        {
                            eprintln!("Error handling UDP packet: {}", e);
                        }
                    });
                }
                Err(e) => {
                    eprintln!("Error receiving UDP packet: {}", e);
                }
            }
        }
    });

    while let Ok((stream, addr)) = tcp_listener.accept().await {
        let min_response_len_tcp_inner = min_response_len_tcp;
        let verbose_inner = verbose;
        tokio::spawn(async move {
            if let Err(e) =
                handle_tcp_connection(stream, addr, min_response_len_tcp_inner, verbose_inner).await
            {
                eprintln!("Error handling connection: {}", e);
            }
        });
    }

    Ok(())
}
