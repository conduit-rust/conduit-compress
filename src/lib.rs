#![crate_name = "conduit-compress"]
#![feature(globs, phase)]

extern crate conduit;
extern crate middleware = "conduit-middleware";
#[phase(plugin, link)] extern crate log;

use middleware::Middleware;
use conduit::{Request, Response};
use std::collections::HashMap;
use std::fmt::Show;

#[deriving(Show, Clone)]
pub struct Compress;

impl middleware::Middleware for Compress {
    fn after(&self, req: &mut Request, res: Result<Response, Box<Show>>)
        -> Result<Response, Box<Show>> {
        match req.headers().find("Accept-Encoding") {
            Some(ref accept_encoding) => {
                res.and_then(|r| {
                    get_compressor(accept_encoding).and_then(|compressor| {
                        match compressor {
                            Gzip => (),
                            Deflate => ()
                        }
                        Err(box "String".to_string() as Box<Show>)
                    })
                })
            },
            // no compression enabled for this request
            None => res
        }
    }
}

#[deriving(Show, Clone)]
pub enum CompresErrorKind {
    NoCompressor
}

#[deriving(Show, Clone)]
pub struct CompressError<S> {
    kind: CompresErrorKind,
    desc: S
}

#[deriving(PartialEq, Eq, Show, Clone, Hash)]
enum Compressor {
    Gzip,
    Deflate
}

fn get_compressor<'a>(accept_encoding: &'a Vec<&'a str>) -> Result<Compressor, Box<Show>> {
    parse_accept_encoding(accept_encoding).and_then(best_encoder)
}

type EncodingPriorities = HashMap<Compressor, f64>;

fn best_encoder(priorities: EncodingPriorities) -> Result<Compressor, Box<Show>> {
    priorities.move_iter().fold((Err(box CompressError {
        kind: NoCompressor,
        desc: "No compressor supported for the requested encodings."
    } as Box<Show>), 0f64), |(best, bestPriority), (compressor, priority)| {
        if priority > bestPriority {
            (Ok(compressor), priority)
        } else {
            (best, bestPriority)
        }
    }).val0()
}

fn parse_accept_encoding<'a>(accept_encodings: &'a Vec<&'a str>) -> Result<EncodingPriorities, Box<Show>> {
    let mut priorities: EncodingPriorities = HashMap::new();
    for encoding in accept_encodings.iter() {
        match encoding.chars().position(|s| s == ';') {
            Some(_) => {
                let split = encoding.split(';').collect::<Vec<&'a str>>();
                if split.len() != 2 {
                    error!("Bad formatting in Accept-Encoding header: {}", encoding);
                } else {
                    parse_encoding(*split.get(0)).map(|c| from_str::<f64>(*split.get(1)).map(|p| {
                        priorities.insert(c, p)
                    }));
                }
            },
            None => { parse_encoding(*encoding).map(|c| priorities.insert(c, 1.0)); }
        }
    }
    Ok(priorities)
}

fn parse_encoding(compressor: &str) -> Option<Compressor> {
    match compressor {
        "gzip" => Some(Gzip),
        "deflate" => Some(Deflate),
        _ => None
    }
}

