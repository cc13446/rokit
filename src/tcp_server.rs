use std::collections::HashMap;
use std::io;
use std::net::{SocketAddr, TcpListener};
use std::sync::{Arc, Mutex};
use std::thread::sleep;
use std::time::Duration;

use crate::rokit_error::RokitError;
use crate::common::parse_ip_port;
use crate::tcp_client::TcpClient;
#[derive(Debug)]
pub struct TcpServer {
    pub socket:SocketAddr,
    tcp_listener:TcpListener,
    pub streams:Arc<Mutex<HashMap<SocketAddr, TcpClient>>>,
    pub closed:Arc<Mutex<bool>>
}

impl Clone for TcpServer {
    fn clone(&self) -> Self {
        let tcp_server_clone = self.tcp_listener.try_clone().unwrap();
        tcp_server_clone.set_nonblocking(true).expect("无法设置为非阻塞模式");
        Self { socket: self.socket.clone(), tcp_listener: tcp_server_clone, streams: self.streams.clone(), closed:self.closed.clone()}
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
                        match t.set_nonblocking(true) {
                            Ok(_) => {},
                            Err(e) => return Err(RokitError::new_msg("TCP监听错误:".to_string() + e.to_string().as_str()))
                        }
                        Ok(TcpServer{
                            socket:res,
                            tcp_listener:t,
                            streams:Arc::new(Mutex::new(HashMap::new())),
                            closed:Arc::new(Mutex::new(false)),
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
    pub fn close (&mut self) {
        let mut close = self.closed.lock().unwrap();
        *close = true;
    }

    pub fn accept(&mut self) -> Result<TcpClient, RokitError> {
        loop {
            let res = self.tcp_listener.accept();
            match res {
                Ok((tcp_stream, socket_addr)) => {
                    match tcp_stream.set_write_timeout(Some(Duration::from_millis(10))) {
                        Ok(_) => {},
                        Err(e) => {
                            return Err(RokitError::new_msg("TCP连接错误:".to_string() + e.to_string().as_str()))
                        }
                    }
                    let tcp_client = TcpClient{tcp_stream, socket_addr};
                    self.streams.lock().unwrap().insert(socket_addr, tcp_client.clone());
                    return Ok(tcp_client);
                },
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                    let close = self.closed.lock().unwrap();
                    if *close {
                        return Err(RokitError::new_msg(format!("TCP服务器关闭:{} {}", self.socket.ip().to_string(), self.socket.port().to_string()).to_string()))
                    }
                    sleep(Duration::from_millis(20));
                    continue;
                }
                Err(e) => return Err(RokitError::new_msg("TCP连接错误:".to_string() + e.to_string().as_str()))
            }
        }
    }
}