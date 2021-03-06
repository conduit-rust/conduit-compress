#![crate_name = "conduit-compress"]
#![feature(globs, phase)]

extern crate conduit;
extern crate middleware = "conduit-middleware";
extern crate flate2;
#[phase(plugin, link)] extern crate log;

use std::fmt::Show;
use middleware::Middleware;
use conduit::{Request, Response};
use parse::{get_compressor, };

mod parse;
mod compressors;

#[deriving(Show, Clone)]
pub struct Compress;

impl middleware::Middleware for Compress {
    fn after(&self, req: &mut Request, res: Result<Response, Box<Show>>) -> Result<Response, Box<Show>> {
        match req.headers().find("Accept-Encoding") {
            Some(ref accept_encoding) => {
                let mut r = try!(res);
                let compressor = try!(get_compressor(accept_encoding));
                match compressor.compress(&mut r) {
                    Ok(_) => Ok(r),
                    Err(e) => Err(e)
                }
            },
            // no compression enabled for this request
            None => res
        }
    }
}

