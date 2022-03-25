use std::io;
use std::net::{SocketAddr, UdpSocket};
use std::sync::{Arc, Mutex};
use std::thread::sleep;
use std::time::Duration;

use crate::rokit_error::RokitError;
use crate::common::parse_ip_port;
#[derive(Debug)]
pub struct UdpClient {
    pub socket_addr:SocketAddr,
    pub udp_stream:UdpSocket,
    pub closed:Arc<Mutex<bool>>
}

#[derive(Debug, Clone)]
pub struct UdpClientReceive {
    pub socket_addr:SocketAddr,
    pub result:String,
}

impl Clone for UdpClient {
    fn clone(&self) -> Self {
        Self { socket_addr: self.socket_addr.clone(), udp_stream: self.udp_stream.try_clone().unwrap(), closed:self.closed.clone() }
    }
}

impl UdpClient {
    pub fn connect(ip:String, port:String) -> Result<Self, RokitError> {
        let socket_addr = parse_ip_port(ip, port);
        match socket_addr  {
            Ok(res) => {
                match UdpSocket::bind("127.0.0.1:34254"){
                    Ok(udp) => {
                        match udp.connect(res) {
                            Ok(_) => {},
                            Err(e) => {
                                return Err(RokitError::new_msg("UDP连接错误:".to_string() + e.to_string().as_str()))
                            }
                        }
                        match udp.set_nonblocking(true) {
                            Ok(_) => {},
                            Err(e) => {
                                return Err(RokitError::new_msg("UDP连接错误:".to_string() + e.to_string().as_str()))
                            }
                        }
                        Ok(UdpClient{
                            socket_addr:res,
                            udp_stream:udp,
                            closed:Arc::new(Mutex::new(false))
                        })
                    },
                    Err(e) => {
                        Err(RokitError::new_msg("UDP连接错误:".to_string() + e.to_string().as_str()))
                    }
                }
            },
            Err(e) => Err(e)
        }
    }

    pub fn send(&mut self, s:String) -> Result<u32, RokitError>{
        match self.udp_stream.send(s.as_bytes()) {
            Ok(x) => Ok(x as u32),
            Err(e) => Err(RokitError::new_msg("UDP写入错误:".to_string() + e.to_string().as_str()))
        }
    }

    pub fn read(&mut self) -> Result<UdpClientReceive, RokitError>{
        loop {
            let mut buffer: [u8;1024]  = [0;1024];
            match self.udp_stream.recv_from(&mut buffer) {
                Ok((i, addr)) => {
                    let closed = self.closed.lock().unwrap();
                    if i == 0 || *closed{
                        return Err(RokitError::new_msg(format!("UDP断开:{} {}", self.socket_addr.ip().to_string(), self.socket_addr.port())));
                    }
                    let res = std::str::from_utf8(&buffer);
                    match res {
                        Ok(s) => {
                            return Ok(UdpClientReceive{socket_addr:addr, result:s.to_string()})
                        },
                        Err(e) => return Err(RokitError::new_msg("UDP转码:".to_string() + e.to_string().as_str()))
                    } 
                },
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                    let closed = self.closed.lock().unwrap();
                    if *closed {
                        return Err(RokitError::new_msg(format!("UDP断开:{} {}", self.socket_addr.ip().to_string(), self.socket_addr.port())));
                    }
                    sleep(Duration::from_millis(10));
                    continue;
                }
                Err(e) => return Err(RokitError::new_msg("UDP读取错误:".to_string() + e.to_string().as_str()))
            }
        }
    }

    pub fn close(&mut self) {
        let mut closed = self.closed.lock().unwrap();
        *closed = true;
    }
}