extern crate futures;
extern crate tokio;

use futures::{Future, Async, Poll};
use std::fmt;

struct HelloWorld;
struct Display<T>(T);

impl Future for HelloWorld {
    type Item = String;
    type Error = ();

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        Ok(Async::Ready("hello world".to_string()))
    }
}

impl<T> Future for Display<T>
where 
    T: Future,
    T::Item: fmt::Display,
{
    type Item = ();
    type Error = T::Error;

    fn poll(&mut self) -> Poll<(), T::Error> {
        let value = match self.0.poll() {
            Ok(Async::Ready(value)) => value,
            Ok(Async::NotReady) => return Ok(Async::NotReady),
            Err(err) => return Err(err),
        };
        println!("{}", value);
        Ok(Async::Ready(()))
    }
}

fn main() {
    let future = Display(HelloWorld);
    tokio::run(future);
}
