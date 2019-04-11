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

/// [x] update to handle both say_hello and get request

///update to handle simple schema
/// get or create simple mleap model that takes said schema

/// update get_product_handler to output score from mleap

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

fn get_product_handler(mut state: State) -> (State, Response<Body>) {
    let res = {
        // Access the `QueryStringExtractor` instance from `state` which was put there for us by the
        // `Router` during request evaluation.
        //
        // As well as permitting storage in `State` by deriving `StateData` our query string
        // extractor struct automatically gains the `take_from` method and a number of other
        // methods via the `gotham::state::FromState` trait.
        //
        // n.b. Once taken out of `state` values can no longer be accessed by other application
        // code or middlewares.
        let query_param = QueryStringExtractor::take_from(&mut state);

        let product = Product {
            name: query_param.name,
        };
        create_response(
            &state,
            StatusCode::OK,
            mime::APPLICATION_JSON,
            serde_json::to_vec(&product).expect("serialized product"),
        )
    };
    (state, res)
}

fn say_hello(state: State) -> (State, Response<Body>) {

    // create the response
    let res = create_response(&state, StatusCode::OK, mime::TEXT_PLAIN, "hello");

    // done!
    (state, res)
}

/// Create a `Router`

fn router() -> Router {
    build_simple_router(|route| {
        route
            .post("/products")
            // This tells the Router that for requests which match this route that query string
            // extraction should be invoked storing the result in a `QueryStringExtractor` instance.
            .with_query_string_extractor::<QueryStringExtractor>()
            .to(get_product_handler);
        route
            .get("/health")
            .to(say_hello);
    })
}

/// Start a server and use a `Router` to dispatch requests
pub fn main() {
    let addr = "127.0.0.1:7878";
    println!("Listening for requests at http://{}", addr);
    gotham::start(addr, router())
}

/// Start a server and call the `Handler` we've defined above for each `Request` we receive.
//pub fn main() {
//    let addr = "127.0.0.1:7878";
//    println!("Listening for requests at http://{}", addr);
//    gotham::start(addr, || Ok(say_hello))
//}
