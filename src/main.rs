extern crate gotham;

#[macro_use]
extern crate gotham_derive;
extern crate hyper;
extern crate mime;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

extern crate uuid;
extern crate semver;
extern crate base64;
extern crate libc;

use hyper::{Body, Response, StatusCode};

use gotham::helpers::http::response::create_response;
use gotham::router::{builder::*, Router};
use gotham::state::{FromState, State};

//pub mod dsl;
//pub mod json;
//pub mod ser;
//pub mod tform;
//pub mod frame;

pub mod bundle;

pub use bundle::* ;

///These are the extracter and the structure that we will use in our app
///to be changed
#[derive(Deserialize, StateData, StaticResponseExtender)]
struct QueryStringExtractor {
    c0: f64,
    c1: f64,
    c2: f64,
    c3: f64,
    c4: f64,
    c5: f64,
    c6: f64,
    c7: f64,
}

fn scoring(query_param : QueryStringExtractor) -> f64 {

    let path = "/tmp/model";
    let builder = ser::FileBuilder::try_new(path).expect("IO error OF");
    let mut registry = ser::Registry::new();

    //LinearRegression_063e4c64538b
    //StandardScaler_5a4909ffc1e2
    //VectorAssembler_18b715378f42

    registry.insert_op(tform::linear_regression::OP);
    registry.insert_op(tform::pipeline::OP);
    registry.insert_op(tform::vector_assembler::OP);
    registry.insert_op(tform::standard_scaler::OP);

    let ctx = ser::Context::new(Box::new(builder), &registry);
    let ctx2 = ctx.try_next("root").expect("ctx error OF");
    let node = ctx2.read_node().expect("another node error OF");

    let mut frame = frame::LeapFrame::with_size(1);

    frame.try_with_doubles("c0".to_string() , vec![query_param.c0] ).expect("Data Error OF");
    frame.try_with_doubles("c1".to_string() , vec![query_param.c1]).expect("Data Error OF");
    frame.try_with_doubles("c2".to_string() , vec![query_param.c2]).expect("Data Error OF");
    frame.try_with_doubles("c3".to_string() , vec![query_param.c3]).expect("Data Error OF");
    frame.try_with_doubles("c4".to_string() , vec![query_param.c4]).expect("Data Error OF");
    frame.try_with_doubles("c5".to_string() , vec![query_param.c5]).expect("Data Error OF");
    frame.try_with_doubles("c6".to_string() , vec![query_param.c6]).expect("Data Error OF");
    frame.try_with_doubles("c7".to_string() , vec![query_param.c7]).expect("Data Error OF");

    //frame.try_with_doubles(String::from("bathrooms"), vec![2.0]).unwrap();
    //frame.try_with_doubles(String::from("bedrooms"), vec![3.0]).unwrap();
    //frame.try_with_doubles(String::from("security_deposit"), vec![50.0]).unwrap();
    //frame.try_with_doubles(String::from("cleaning_fee"), vec![30.0]).unwrap();
    //frame.try_with_doubles(String::from("extra_people"), vec![0.0]).unwrap();
    //frame.try_with_doubles(String::from("number_of_reviews"), vec![23.0]).unwrap();
    //frame.try_with_doubles(String::from("square_feet"), vec![1200.0]).unwrap();
    //frame.try_with_doubles(String::from("review_scores_rating"), vec![93.0]).unwrap();

    //frame.try_with_strings(String::from("cancellation_policy"), vec![String::from("strict")]).unwrap();
    //frame.try_with_strings(String::from("host_is_superhost"), vec![String::from("1.0")]).unwrap();
    //frame.try_with_strings(String::from("instant_bookable"), vec![String::from("1.0")]).unwrap();
    //frame.try_with_strings(String::from("room_type"), vec![String::from("Entire home/apt")]).unwrap();
    //frame.try_with_strings(String::from("state"), vec![String::from("NY")]).unwrap();

    node.transform(&mut frame).expect("Node Transform Error OF");

    let r = frame.get_doubles("prediction").and_then(|x| x.first()).expect("get pred error OF");
    *r
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

        let score = scoring(query_param);

        //let product = Product {
        //    c0: query_param.c0,
        //    c1: query_param.c1,
        //    c2: query_param.c2,
        //    c3: query_param.c3,
        //    c4: query_param.c4,
        //    c5: query_param.c5,
        //    c6: query_param.c6,
        //    c7: query_param.c7,
        //};

        create_response(
            &state,
            StatusCode::OK,
            mime::APPLICATION_JSON,
            serde_json::to_vec(&score).expect("serialized product"),
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
//future should get back to post request
fn router() -> Router {
    build_simple_router(|route| {
        route
            .get("/products")
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
