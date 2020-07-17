
use reqwest::{Response, Request};

pub trait IntoRequest {
    type ResponseType: FromResponse;

    fn into_request(self) -> Request;
}

pub trait FromResponse: Sized {
    type Error: Sized;
    
    fn from_response(response: Response) -> Result<Self, Self::Error>;
}