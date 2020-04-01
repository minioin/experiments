// TODO: Design a method get_by(property) and find_by(property). The properties should be declarative
// so that each implementation can choose to optimmize it according to its needs.
// The design should accept a function that filters the result. However, the function should be able to
// do more than just simple comparision, including, specific implementation of getting values from cache.
pub trait Predicate<T>: PartialEq {
	fn filter(&self, item: T) -> bool;
}

pub trait Filterable<T> : Store {
	fn find_by(&self, predicate: impl Predicate<T>) -> Option<&[&T]>;
}