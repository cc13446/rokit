use crate::rokit_error::RokitError;
use crate::common::parse_ip_port;
use std::{net::{SocketAddr, TcpStream, Shutdown}, io::{Write, Read}, str, time::Duration};
#[derive(Debug)]
pub struct TcpClient {
    pub socket_addr:SocketAddr,
    pub tcp_stream:TcpStream
}

impl Clone for TcpClient {
    fn clone(&self) -> Self {
        Self { socket_addr: self.socket_addr.clone(), tcp_stream: self.tcp_stream.try_clone().unwrap() }
    }
}

impl TcpClient {
    pub fn connect(ip:String, port:String) -> Result<Self, RokitError> {
        let socket_addr = parse_ip_port(ip, port);
        match socket_addr  {
            Ok(res) => {
                let tcp = TcpStream::connect(res);
                match tcp {
                    Ok(t) => {
                        match t.set_write_timeout(Some(Duration::from_millis(10))) {
                            Ok(_) => {},
                            Err(e) => {
                                return Err(RokitError::new_msg("TCP连接错误:".to_string() + e.to_string().as_str()))
                            }
                        }
                        Ok(TcpClient{
                            socket_addr:res,
                            tcp_stream:t
                        })
                    },
                    Err(e) => {
                        Err(RokitError::new_msg("TCP连接错误:".to_string() + e.to_string().as_str()))
                    }
                }
            },
            Err(e) => Err(e)
        }
    }

    pub fn send(&mut self, s:String) -> Result<u32, RokitError>{
        match self.tcp_stream.write(s.as_bytes()) {
            Ok(x) => Ok(x as u32),
            Err(e) => Err(RokitError::new_msg("TCP写入错误:".to_string() + e.to_string().as_str()))
        }
    }

    pub fn read(&mut self) -> Result<String, RokitError>{
        let mut buffer: [u8;1024]  = [0;1024];
        match self.tcp_stream.read(&mut buffer) {
            Ok(x) => {
                if x == 0 {
                    return  Err(RokitError::new_msg(format!("TCP断开{} {}", self.socket_addr.ip().to_string(), self.socket_addr.port())));
                }
                let res = str::from_utf8(&buffer);
                match res {
                    Ok(s) => {
                        Ok(s.to_string())
                    },
                    Err(e) => Err(RokitError::new_msg("TCP转码:".to_string() + e.to_string().as_str()))
                } 
            },
            Err(e) => Err(RokitError::new_msg("TCP读取错误:".to_string() + e.to_string().as_str()))
        }
    }

    pub fn disconnect(&mut self) -> Result<(), RokitError>{
        match self.tcp_stream.shutdown(Shutdown::Both) {
            Ok(x) => Ok(x),
            Err(e) => Err(RokitError::new_msg("TCP断开错误:".to_string() + e.to_string().as_str()))
        }
    }
}