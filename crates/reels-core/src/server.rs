use lunatic::{net, net::ToSocketAddrs, Mailbox, Process};
use std::io::{self, BufReader, BufWriter};
use std::net::SocketAddr;

use crate::http::HttpRequest;
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

    pub fn bind<S: ToSocketAddrs>(mut self, address: S) -> io::Result<Self> {
        self.address = Some(address.to_socket_addrs()?.next().unwrap());
        Ok(self)
    }

    pub fn start(self) {
        // TODO: refactor this and add timeout for keep-alive
        if let Some(address) = self.address {
            let listener = net::TcpListener::bind(address).unwrap();
            while let Ok((tcp_stream, _peer)) = listener.accept() {
                println!("Accepted connection");
                // Handle connections in a new process
                Process::spawn(
                    (tcp_stream, self.router.clone()),
                    |(tcp_stream, router), _: Mailbox<()>| {
                        println!("Process started");
                        let mut buf_reader = BufReader::with_capacity(4198, tcp_stream.clone());
                        let mut buf_writer = BufWriter::new(tcp_stream);
                        while let Some(request) = HttpRequest::parse(&mut buf_reader).unwrap() {
                            println!("{}", request.url);
                            println!(
                                "{:?}",
                                request
                                    .url
                                    .path_segments()
                                    .unwrap()
                                    .collect::<Vec<&str>>()
                                    .join("/")
                            );
                            let response = router.route(request);
                            // println!("{:#?}", &response);
                            response.write(&mut buf_writer).unwrap();
                            println!("Response written");
                        }
                        println!("Socket closed");
                    },
                );
            }
        }
    }
}
