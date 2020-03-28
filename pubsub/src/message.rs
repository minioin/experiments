use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Topic {
	HI,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
	pub topic: Topic,
	pub data: Vec<u8>,
}

impl Message {
	pub fn new(topic: Topic, data: Vec<u8>) -> Self {
		Self { topic, data }
	}
}
