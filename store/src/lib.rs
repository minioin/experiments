// TODO: Provide a proper memory store implementation as an experiment.

pub type BoxedStore<T> = Box<dyn Store<Item = T>>;
pub type StoreError = Box<dyn std::error::Error>;

pub trait Store: Iterator {
	fn add(&mut self, data: Self::Item) -> Result<(), StoreError>;
	fn get(&self, id: usize) -> Option<Self::Item>;
	fn update(&mut self, item: Self::Item) -> Result<(), StoreError>;
	fn delete(&mut self, id: usize) -> Result<(), StoreError>;
}