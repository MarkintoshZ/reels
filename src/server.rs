use lunatic::{net, Mailbox, Process};
use std::io::{BufReader, BufWriter};
use std::net::SocketAddr;

use crate::http::{request::HttpRequest, response::HttpResponse, status::StatusCode};
use crate::router::Router;

pub struct Server {
    address: Option<SocketAddr>,
    router: Router,
}

impl Server {
    pub fn new(router: Router) -> Self {
        Server {
            address: None,
            router,
        }
    }

    pub fn bind(mut self, address: SocketAddr) -> Self {
        self.address = Some(address);
        self
    }

    pub fn start(self) {
        // TODO: refactor this and add timeout for keep-alive
        if let Some(address) = self.address {
            let listener = net::TcpListener::bind(address).unwrap();
            while let Ok((tcp_stream, _peer)) = listener.accept() {
                println!("Accepted connection");
                // Handle connections in a new process
                Process::spawn(tcp_stream, |tcp_stream, _: Mailbox<()>| {
                    println!("Process started");
                    let mut buf_reader = BufReader::with_capacity(4198, tcp_stream.clone());
                    let mut buf_writer = BufWriter::new(tcp_stream);
                    while let Some(request) = HttpRequest::parse(&mut buf_reader).unwrap() {
                        println!("{:#?}", request);
                        let res = match &request.url[..] {
                            "/" => HttpResponse::builder()
                                .header(
                                    "content-type".to_owned(),
                                    "text/html; charset=utf-8".to_owned(),
                                )
                                .body("Hello world!".to_owned())
                                .finalize(),
                            _ => HttpResponse::builder()
                                .status(StatusCode::NOT_FOUND)
                                .header(
                                    "content-type".to_owned(),
                                    "text/html; charset=utf-8".to_owned(),
                                )
                                .body("Oops... Looks like you are lost".to_owned())
                                .finalize(),
                        };
                        println!("{:#?}", res);
                        res.write(&mut buf_writer).unwrap();
                        println!("Response written");
                    }
                    println!("Socket closed");
                });
            }
        }
    }
}
