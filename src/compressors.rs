use conduit::Response;

use std::from_str::FromStr;
use std::io::MemReader;

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
    pub fn compress(&self, res: &mut Response) {
        res.body = box MemReader::new(res.body.read_to_end().unwrap()) as Box<Reader + Send>;
    }
}

