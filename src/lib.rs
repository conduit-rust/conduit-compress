#![crate_name = "conduit-compress"]
#![feature(globs)]

extern crate conduit;
extern crate middleware = "conduit-middleware";

use middleware::Middleware;
use conduit::{Request, Response};

#[deriving(Show, Clone)]
struct Compress;

impl middleware::Middleware for Compress {
    fn after(&self, req: &mut Request, res: Result<Response, Box<Show>>)
        -> Result<Response, Box<Show>> {
        match req.headers.find("Accept-Encoding") {
            Some(ref accept_encoding) => {
                res.map(|r| {
                    // do some compression
                })
            },
            // no compression enabled for this request
            None => res
        }
    }
}

