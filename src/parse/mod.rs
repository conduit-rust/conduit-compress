use std::collections::HashMap;
use std::fmt::Show;
use super::compressors::Compressor;

#[deriving(Show, Clone)]
enum CompressErrorKind {
    NoCompressor
}

#[deriving(Show, Clone)]
struct CompressError<S> {
    kind: CompressErrorKind,
    desc: S
}


pub fn get_compressor<'a>(accept_encoding: &'a Vec<&'a str>) -> Result<Compressor, Box<Show>> {
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
                    from_str(split[0]).map(|c| from_str::<f64>(split[1].slice_from(2)).map(|p| {
                        priorities.insert(c, p)
                    }));
                }
            },
            None => { from_str(*encoding).map(|c| priorities.insert(c, 1.0)); }
        }
    }
    Ok(priorities)
}

#[cfg(test)]
mod test {
    use super::{parse_accept_encoding, best_encoder, get_compressor, EncodingPriorities};
    use super::super::compressors::{Compressor, Gzip, Deflate};

    fn to_priorities(priorities: Vec<(Compressor, f64)>) -> EncodingPriorities {
        priorities.move_iter().collect()
    }

    #[test]
    fn parse_no_q() {
        let test_accept_encoding = vec!["gzip", "deflate"];
        assert_eq!(parse_accept_encoding(&test_accept_encoding).ok().unwrap(),
                   to_priorities(vec![(Gzip, 1.0), (Deflate, 1.0)]));
    }

    #[test]
    fn parse_with_q() {
        let test_accept_encoding = vec!["gzip;q=0.8", "deflate"];
        assert_eq!(parse_accept_encoding(&test_accept_encoding).ok().unwrap(),
                   to_priorities(vec![(Gzip, 0.8), (Deflate, 1.0)]));
    }

    #[test]
    fn best_encoder_detection() {
        assert_eq!(best_encoder(to_priorities(vec![(Gzip, 0.8), (Deflate, 1.0)])).ok().unwrap(), Deflate);
    }

    #[test]
    fn end_to_end() {
        let test_accept_encoding = vec!["gzip;q=0.8", "deflate"];
        assert_eq!(get_compressor(&test_accept_encoding).ok().unwrap(), Deflate);
    }
}

