extern crate gotham;

use gotham::state::State;

#[macro_use]
extern crate gotham_derive;
extern crate hyper;
extern crate mime;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use hyper::{Body, Response, StatusCode};

use gotham::helpers::http::response::create_response;
use gotham::router::{builder::*, Router};
use gotham::state::{FromState, State};

///update main to handle both say_hello and get request
///


const HELLO_WORLD: &'static str = "Hello World!";

///These are the extracter and the structure that we will use in our app
///to be changed
#[derive(Deserialize, StateData, StaticResponseExtender)]
struct QueryStringExtractor {
    name: String,
}
/// A Product
#[derive(Serialize)]
struct Product {
    name: String,
}

/// Create a `Handler` which is invoked when responding to a `Request`.
///
/// How does a function become a `Handler`?.
/// We've simply implemented the `Handler` trait, for functions that match the signature used here,
/// within Gotham itself.
pub fn say_hello(state: State) -> (State, &'static str) {
    (state, HELLO_WORLD)
}

/// Start a server and call the `Handler` we've defined above for each `Request` we receive.
pub fn main() {
    let addr = "127.0.0.1:7878";
    println!("Listening for requests at http://{}", addr);
    gotham::start(addr, || Ok(say_hello))
}