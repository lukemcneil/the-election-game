use crate::types::{BadRequest, Error, Guess};
use rocket::{
    http::{ContentType, Status},
    request::Request,
    response, Response,
};
use std::{
    hash::{Hash, Hasher},
    io::Cursor,
};

// TODO: This is terrible and should be removed but looks like HashSet does not impl Hash
impl Hash for Guess {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.player.hash(state);
    }
}

impl PartialEq for Guess {
    fn eq(&self, other: &Self) -> bool {
        self.player == other.player
    }
}

impl Eq for Guess {}

// Convert our custom Error type into HTTP responses
impl<'r> response::Responder<'r> for Error {
    fn respond_to(self, _: &Request) -> response::Result<'r> {
        let body = BadRequest::new(self);
        let body = serde_json::to_string(&body).expect("to BadRequest serialize");
        Ok(Response::build()
            .status(Status::BadRequest)
            .header(ContentType::JSON)
            .sized_body(Cursor::new(body))
            .finalize())
    }
}
