use std::{
	net::{Ipv4Addr, UdpSocket},
	time::Duration,
};

use net2::UdpBuilder;

use crate::channel::Channel;

const MULTICAST_ADDR: &'static str = "233.255.255.255:20000";

pub struct MulticastNetwork {
	pub socket: UdpSocket,
}

impl Channel<Vec<u8>> for MulticastNetwork {
	fn send(&mut self, msg: Vec<u8>) {
		self.socket.send_to(&msg, MULTICAST_ADDR);
	}

	fn recv(&self) -> Option<Vec<u8>> {
		let mut buf = [0u8; 64];
		match self.socket.recv_from(&mut buf) {
			Ok((len, remote_addr)) => {
				let data = &buf[..len];
				Some(Vec::from(data))
			},
			Err(err) => None,
		}
	}
}

impl MulticastNetwork {
	pub fn new(addr: &str) -> Self {
		let udp_socket = UdpBuilder::new_v4().unwrap();
		udp_socket.reuse_address(true).unwrap();
		let socket = udp_socket.bind("0.0.0.0:20000").unwrap();
		socket
			.set_read_timeout(Option::from(Duration::from_millis(10)))
			.unwrap();
		socket
			.join_multicast_v4(
				&Ipv4Addr::new(233, 255, 255, 255),
				&Ipv4Addr::new(0, 0, 0, 0),
			)
			.unwrap();
		Self { socket }
	}
}
