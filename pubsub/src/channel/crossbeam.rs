use crossbeam::channel::{Receiver, Sender};

use crate::channel::Channel;

pub struct CrossbeamSocket<T> {
	pub rx: Receiver<T>,
	pub tx: Sender<T>,
}

impl<T> Channel<T> for CrossbeamSocket<T> {
	fn send(&mut self, msg: T) {
		self.tx.send(msg);
	}

	fn recv(&self) -> Option<T> {
		self.rx.recv().ok()
	}
}

impl<T> CrossbeamSocket<T> {
	pub fn pair() -> (Self, Self) {
		let (tx1, rx1) = crossbeam::channel::unbounded();
		let (tx2, rx2) = crossbeam::channel::unbounded();
		(Self { tx: tx1, rx: rx2 }, Self { tx: tx2, rx: rx1 })
	}

	pub fn with(tx: Sender<T>, rx: Receiver<T>) -> Self {
		Self { rx, tx }
	}

	pub fn bind_with() {}
}

impl<T> Clone for CrossbeamSocket<T> {
	fn clone(&self) -> Self {
		Self {
			rx: self.rx.clone(),
			tx: self.tx.clone(),
		}
	}
}
