use rustful::filter::{ FilterContext, ResponseFilter, ResponseAction };
use rustful::response::Data;
use rustful::StatusCode;
use rustful::header::Headers;
use rustful::header::ContentType;
use rustful::mime::{ Mime, TopLevel, SubLevel };

pub struct JsonSerializer;

impl ResponseFilter for JsonSerializer {
    fn begin(&self, ctx: FilterContext, status: StatusCode, mut headers: Headers) -> (StatusCode, Headers, ResponseAction) {
        //Set Content-Type: application/jSON
        headers.set::<ContentType>(ContentType(Mime(TopLevel::Application, SubLevel::Json, vec![])));
        
        (status, headers, ResponseAction::next(None::<String>))
    }

    fn write<'a>(&'a self, _ctx: FilterContext, bytes: Option<Data<'a>>) -> ResponseAction {
        //todo: Get response *object* and convert into JSON
        
        //http://doc.rust-lang.org/rustc-serialize/rustc_serialize/json/index.html
        //let encoded = json::encode(&object).unwrap();
    
        ResponseAction::next(bytes)
    }

    fn end(&self, ctx: FilterContext) -> ResponseAction {
        //Do nothing
        ResponseAction::next(None::<String>)
    }
}