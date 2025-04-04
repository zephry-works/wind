use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Iface {
   pub name:  String,
   pub ipv4:  Option<Ipv4Addr>,
   pub ipv6:  Option<Ipv6Addr>,
   pub index: u32,
}

#[derive(Debug, Clone, Default)]
pub enum Stack {
   #[default]
   V4,
   V6,
}
impl From<&SocketAddr> for Stack {
   fn from(value: &SocketAddr) -> Self {
      match value {
         SocketAddr::V4(..) => Self::V4,
         SocketAddr::V6(..) => Self::V6,
      }
   }
}
impl From<&IpAddr> for Stack {
   fn from(value: &IpAddr) -> Self {
      match value {
         IpAddr::V4(..) => Self::V4,
         IpAddr::V6(..) => Self::V6,
      }
   }
}

#[derive(Serialize, Debug, Clone, Copy)]
pub enum Network {
   TCP,
   UDP,
   ICMPv4,
   ICMPv6,
}

#[derive(Debug, Clone, Copy)]
pub enum StackPrefer {
   V4,
   V6,
   V4V6,
   V6V4,
}

impl StackPrefer {
   pub fn support_v6(&self) -> bool {
      match self {
         StackPrefer::V4 => false,
         _ => true,
      }
   }
}
