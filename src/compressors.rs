use conduit::Response;
use flate2::{GzEncoder, DeflateEncoder, Default};

use std::from_str::FromStr;
use std::fmt::Show;
use std::io::{MemReader, MemWriter};
use std::io::util::copy;

#[deriving(PartialEq, Eq, Show, Clone, Hash)]
pub enum Compressor {
    Gzip,
    Deflate
}

impl FromStr for Compressor {
    fn from_str(s: &str) -> Option<Compressor> {
        match s {
            "gzip" => Some(Gzip),
            "deflate" => Some(Deflate),
            _ => None
        }
    }
}

impl Compressor {
    pub fn compress(&self, res: &mut Response) -> Result<(), Box<Show>> {
        let mut compressed;

        {
            let body: &mut Reader = res.body;
            match *self {
                Gzip => {
                    let mut compressor = GzEncoder::new(MemWriter::new(), Default);
                    try!(copy_from_body(body, &mut compressor));
                    compressed = compressor.finish().ok().unwrap().unwrap();
                },
                Deflate => {
                    let mut compressor = DeflateEncoder::new(MemWriter::new(), Default);
                    try!(copy_from_body(body, &mut compressor));
                    compressed = compressor.finish().ok().unwrap().unwrap();
                }
            }
        }

        res.body = box MemReader::new(compressed) as Box<Reader + Send>;
        Ok(())
    }
}

fn copy_from_body<W: Writer>(mut body: &mut Reader,
                             compressor: &mut W) -> Result<(), Box<Show>> {
    copy(&mut body, compressor).map_err(|e| box e as Box<Show>)
}

