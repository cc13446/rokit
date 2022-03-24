use super::rokit_error::RokitError;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpStream};
pub struct TcpClient {
    socket:SocketAddr,
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
                    _ => return Err(RokitError{msg:"IP地址格式错误".to_string() + ip.clone().as_str()})
                }
                
            }
            let parse_port = match port.parse::<u16>() {
                Ok(x) => x,
                _ => return Err(RokitError{msg:"端口格式错误".to_string() + port.clone().as_str()})
            };

            if parse_ip.len() != 4 {
                return Err(RokitError{msg:"IP地址格式错误".to_string() + ip.clone().as_str()});
            }
            let socket_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(parse_ip[0], parse_ip[1], parse_ip[2], parse_ip[3])), parse_port);
            let tcp = TcpStream::connect(socket_addr);
            match tcp {
                Ok(t) => {
                    Ok(TcpClient{
                        socket:socket_addr,
                        tcp:t
                    })
                },
                Err(e) => {
                    Err(RokitError{msg:"tcp连接错误".to_string() + e.to_string().as_str()})
                }
            }
        } else {
            Err(RokitError{msg:"IP地址格式错误".to_string() + ip.clone().as_str()})
        }
    } 
}