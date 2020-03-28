use pubsub::node::{LocalMessage, LocalNode};
use std::time::Duration;

fn main() {
	let mut node = LocalNode::new();
	let thread = node.start();
	let topic: Vec<u8> = "hello".into();
	node.subscribe(topic.clone());
	std::thread::sleep(Duration::from_secs(3));
	node.publish(LocalMessage::BROADCAST(topic.clone()));

	thread.join();
}

