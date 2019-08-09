#![feature(custom_attribute, async_await, param_attrs)]
#![allow(unused_attributes)]

pub mod req;
pub mod resp;

use interfacer_http::{content_types, http::header::COOKIE, http_interface, Response, Result};

#[http_interface]
trait JWInterface: Clone {
    //    #[put("/api/user/{id}?age={age}")]
    //    #[expect(200, content_types::APPLICATION_JSON)]
    //    async fn put_user(
    //        &self,
    //        id: u64,
    //        age: i32,
    //        #[body] user: &User,
    //        #[header(COOKIE)] cookie: &str
    //    ) -> Result<Response<User>>;
}
