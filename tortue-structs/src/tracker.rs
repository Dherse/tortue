extern crate tortue_reqbuilder;

use serde::Deserialize;
use tortue_reqtraits::{IntoRequest, FromResponse};

#[derive(FromResponse, Deserialize)]
#[deserialize(tortue_bencode::from_bytes)]
struct TrackResponse {

}

#[derive(IntoRequest)]
#[req(response = TrackResponse)]
struct TrackRequest {

}