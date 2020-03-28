pub mod crossbeam;
pub mod multicast;

pub trait Channel<T> {
	fn send(&mut self, msg: T);
	fn recv(&self) -> Option<T>;
}
