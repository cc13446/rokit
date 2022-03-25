use std::collections::HashMap;
use std::net::{SocketAddr, TcpListener};
use std::sync::{Arc, Mutex};

use crate::rokit_error::RokitError;
use crate::common::parse_ip_port;
use crate::tcp_client::TcpClient;
#[derive(Debug)]
pub struct TcpServer {
    pub socket:SocketAddr,
    tcp_listener:TcpListener,
    pub streams:Arc<Mutex<HashMap<SocketAddr, TcpClient>>>
}

impl Clone for TcpServer {
    fn clone(&self) -> Self {
        Self { socket: self.socket.clone(), tcp_listener: self.tcp_listener.try_clone().unwrap(), streams: self.streams.clone() }
    }
}

impl TcpServer {
    pub fn listen(ip:String, port:String) -> Result<Self, RokitError> {
        let socket_addr = parse_ip_port(ip, port);
        match socket_addr  {
            Ok(res) => {
                let listener = TcpListener::bind(res);
                match listener {
                    Ok(t) => {
                        Ok(TcpServer{
                            socket:res,
                            tcp_listener:t,
                            streams:Arc::new(Mutex::new(HashMap::new()))
                        })
                    },
                    Err(e) => {
                        Err(RokitError::new_msg("TCP监听错误:".to_string() + e.to_string().as_str()))
                    }
                }
            },
            Err(e) => Err(e)
        }
    }

    pub fn accept(&mut self) -> Result<TcpClient, RokitError> {
        let res = self.tcp_listener.accept();
        match res {
            Ok((tcp_stream, socket_addr)) => {
                let tcp_client = TcpClient{tcp_stream, socket_addr};
                self.streams.lock().unwrap().insert(socket_addr, tcp_client.clone());
                Ok(tcp_client)
            },
            Err(e) => Err(RokitError::new_msg("TCP连接错误:".to_string() + e.to_string().as_str()))
        }
    }
}