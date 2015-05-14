//Include macros to be able to use `inser_routes!`.
#[macro_use]
extern crate rustful;

use std::error::Error;

mod context_filters;
mod response_filters;

use rustful::{ Server, Context, Response, TreeRouter, Handler, StatusCode };
use rustful::Method::{ Get, Post, Delete };
use rustful::response::Data;
use rustful::header::Headers;
use rustful::mime::{ Mime, TopLevel, SubLevel };

fn root(context: Context, response: Response) {
    if let Err(e) = response.into_writer().send("Root") {
        context.log.note(&format!("could not send response: {}", e.description()));
    }
}

fn not_found(context: Context, mut response: Response) {
    response.set_status(StatusCode::NotFound);
}

//Dodge an ICE, related to functions as handlers.
struct HandlerFn(fn(Context, Response));
impl Handler for HandlerFn {
    fn handle_request(&self, context: Context, response: Response) {
        self.0(context, response);
    }
}

fn main() {
    //Build and run the server.
    let server_result = Server {
        //Turn a port number into an IPV4 host address (0.0.0.0:8080 in this case).
        host: 8080.into(),

        //Create a TreeRouter and fill it with handlers.
        handlers: insert_routes! {
            TreeRouter::new() => {
                "/" => Get: HandlerFn(root),
                "/*" => Get: HandlerFn(not_found)
                
                // "/authentication" => Get: HandlerFn(say_hello),
                // "/authentication" => Post: HandlerFn(say_hello),
                // "/authentication" => Delete: HandlerFn(say_hello)
            }
        },
        
        context_filters: vec![
            //Default to Accepts:application/json in requests
            Box::new(context_filters::DefaultContentType::new(Mime(TopLevel::Application, SubLevel::Json, vec![])))
        ],
        
        response_filters: vec![
            //Decide on a content type for the response
            //Box::new(ContentNegotiator),
        
            //Serialize to JSON (if headers are set)
            Box::new(response_filters::JsonSerializer)
        ],

        //Use default values for everything else.
        ..Server::default()
    }.run();

    match server_result {
        Ok(_server) => {},
        Err(e) => println!("could not start server: {}", e.description())
    }
}