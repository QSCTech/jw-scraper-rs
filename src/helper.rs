use interfacer_http::http::{request::Builder, header::HOST};

pub const JWB_HOST: &str = "jwbinfosys.zju.edu.cn";

pub fn request_initializer() -> Builder {
    let mut builder = Builder::new();
    builder.header(HOST, JWB_HOST);
    builder
}