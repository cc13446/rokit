use super::rokit_error::RokitError;
use std::{net::{IpAddr, Ipv4Addr, SocketAddr, TcpStream, Shutdown}, io::{Write, Read}, str};
pub struct TcpClient {
    pub socket:SocketAddr,
    tcp:TcpStream
}

impl TcpClient {
    pub fn connect(ip:String, port:String) -> Result<Self, RokitError> {
        let split_ip : Vec<&str> = ip.as_str().split(".").collect();
        if split_ip.len() == 4 {
            let mut parse_ip : Vec<u8> = Vec::new();
            for s in split_ip {
                let temp = s.parse::<u8>();
                match temp {
                    Ok(x) => parse_ip.push(x),
                    _ => return Err(RokitError::new_msg("IP地址格式错误:".to_string() + ip.clone().as_str()))
                }
                
            }
            let parse_port = match port.parse::<u16>() {
                Ok(x) => x,
                _ => return Err(RokitError::new_msg("端口格式错误:".to_string() + port.clone().as_str()))
            };

            if parse_ip.len() != 4 {
                return Err(RokitError::new_msg("IP地址格式错误:".to_string() + ip.clone().as_str()))
            }
            let socket_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(parse_ip[0], parse_ip[1], parse_ip[2], parse_ip[3])), parse_port);
            let tcp = TcpStream::connect(socket_addr);
            match tcp {
                Ok(t) => {
                    match t.set_nonblocking(true) {
                        Ok(_) => {},
                        Err(e) => return Err(RokitError::new_msg("TCP创建错误:".to_string() + e.to_string().as_str()))
                    }
                    Ok(TcpClient{
                        socket:socket_addr,
                        tcp:t
                    })
                },
                Err(e) => {
                    Err(RokitError::new_msg("TCP连接错误:".to_string() + e.to_string().as_str()))
                }
            }
        } else {
            Err(RokitError::new_msg("IP地址格式错误:".to_string() + ip.clone().as_str()))
        }
    }

    pub fn send(&mut self, s:String) -> Result<u32, RokitError>{
        match self.tcp.write(s.as_bytes()) {
            Ok(x) => Ok(x as u32),
            Err(e) => Err(RokitError::new_msg("TCP写入错误:".to_string() + e.to_string().as_str()))
        }
    }

    pub fn read(&mut self) -> Result<String, RokitError>{
        let mut buffer: [u8;1024]  = [0;1024];
        match self.tcp.read(&mut buffer) {
            Ok(x) => {
                if x == 0 {
                    return  Err(RokitError::new_msg(format!("TCP断开{} {}", self.socket.ip().to_string(), self.socket.port())));
                }
                let res = str::from_utf8(&buffer);
                match res {
                    Ok(s) => {
                        Ok(s.to_string())
                    },
                    Err(e) => Err(RokitError::new_msg("TCP读取错误:".to_string() + e.to_string().as_str()))
                } 
            },
            Err(_) => Err(RokitError::new(true, "".to_string()))
        }
    }

    pub fn disconnect(&mut self) -> Result<(), RokitError>{
        match self.tcp.shutdown(Shutdown::Both) {
            Ok(x) => Ok(x),
            Err(e) => Err(RokitError::new_msg("TCP断开错误:".to_string() + e.to_string().as_str()))
        }
    }
}