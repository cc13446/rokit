
use std::net::{SocketAddr, IpAddr, Ipv4Addr};

use crate::rokit_error::RokitError;

pub fn parse_ip_port(ip:String, port:String) -> Result<SocketAddr, RokitError> {
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
        Ok(socket_addr)
    } else {
        Err(RokitError::new_msg("IP地址格式错误:".to_string() + ip.clone().as_str()))
    }
}

pub fn ascii_to_utf_8(input:String) -> Result<String, RokitError> {
    let split_input : Vec<&str> = input.as_str().split(",").collect();
    let mut buffer : Vec<u8> = Vec::new();
    for s in split_input {
        let temp = s.parse::<u8>();
        match temp {
            Ok(x) => buffer.push(x),
            Err(_) => return Err(RokitError::new_msg("用户输入错误, 格式为'65,66,67' => 'ABC'".to_string()))
        }
    }
    Ok(String::from_iter(buffer.iter().map(|v| { *v as char })))
}
