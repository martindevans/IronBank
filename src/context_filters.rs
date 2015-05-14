use rustful::filter::{ ContextFilter, ContextAction, FilterContext };
use rustful::context::Context;
use rustful::header::{ Accept, Quality, QualityItem };
use rustful::mime::Mime;

//Filter which checks if the client has set the accepts header. If not, sets it to a default value
pub struct DefaultContentType {
    content_type: Mime
}
impl DefaultContentType {
    pub fn new(content_type: Mime) -> DefaultContentType {
        DefaultContentType {
            content_type: content_type
        }
    }
}
impl ContextFilter for DefaultContentType {
    fn modify(&self, _ctx: FilterContext, context: &mut Context) -> ContextAction {
        if !context.headers.has::<Accept>() {
            context.headers.set::<Accept>(Accept(vec![
                QualityItem::new(self.content_type.clone(), Quality(0)) ]
            ));
        }
    
        ContextAction::next()
    }
}