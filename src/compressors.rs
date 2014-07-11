#[deriving(PartialEq, Eq, Show, Clone, Hash)]
pub enum Compressor {
    Gzip,
    Deflate
}

