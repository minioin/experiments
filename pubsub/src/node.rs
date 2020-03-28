use std::{
	collections::{HashMap, HashSet},
	sync::{Arc, RwLock},
	thread::JoinHandle,
};

use crossbeam::channel::{Receiver, Sender};

use crate::{
	channel::{
		crossbeam::CrossbeamSocket, multicast::MulticastNetwork, Channel,
	},
	message::Message,
};

pub trait Node {
	fn start(&mut self);
	fn join(&mut self) -> Box<dyn Channel<Vec<u8>>>;
	fn leave(&mut self);
}

pub struct SharedData {
	subscriptions: HashSet<Vec<u8>>,
}

impl Default for SharedData {
	fn default() -> Self {
		Self {
			subscriptions: HashSet::new(),
		}
	}
}

/// Publish Subscribe pattern implemented over a generic channel.
pub struct LocalNode {
	thread_channel: (Sender<LocalMessage>, Option<Receiver<LocalMessage>>),
	network: Option<MulticastNetwork>,
	subscriptions: HashMap<Vec<u8>, CrossbeamSocket<LocalMessage>>,
	pub data: Arc<RwLock<SharedData>>,
}

#[derive(Debug)]
pub enum LocalMessage {
	LEAVE,
	SUBSCRIBE(Vec<u8>),
	UNSUBSCRIBE(Message),
	BROADCAST(Vec<u8>),
	KILL,
}

impl LocalNode {
	pub fn new() -> Self {
		let (tx1, rx1) = crossbeam::channel::unbounded();
		Self {
			subscriptions: HashMap::new(),
			thread_channel: (tx1, Some(rx1)),
			network: Some(MulticastNetwork::new("")),
			data: Arc::new(RwLock::new(SharedData::default())),
		}
	}

	pub fn start(&mut self) -> JoinHandle<()> {
		let rx = self.thread_channel.1.take().unwrap();
		let mut network = self.network.take().unwrap();
		std::thread::spawn(move || {
			loop {
				if let Some(msg) = network.recv() {
					println!("Got network msg");
					let msg: Option<Message> =
						serde_cbor::from_slice(&msg).ok();
					// TODO: Handle incoming messages
					if let Some(msg) = msg {
						match msg.topic {
							_ => {},
						}
					}
				}

				// Lets see if we got any message from local threads
				if let Some(msg) = rx.try_recv().ok() {
					println!("Got local msg: {:?}", msg);
					match msg {
						LocalMessage::KILL => {
							println!("Exiting");
							break;
						},
						LocalMessage::BROADCAST(msg) => {
							println!("Broadcasting message to network");
							network.send(msg);
						},
						_ => {},
					}
				}
			}
		})
	}

	pub fn subscribe(&mut self, topic: Vec<u8>) {
		let mut data = self.data.write().unwrap();
		data.subscriptions.insert(topic.clone());
	}

	pub fn publish(&mut self, msg: LocalMessage) {
		self.thread_channel.0.send(msg);
	}

	pub fn stop(self) {
		self.thread_channel.0.send(LocalMessage::KILL);
		// std::mem::drop(self)
	}
}
