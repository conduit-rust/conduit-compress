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
                    from_str(*split.get(0)).map(|c| from_str::<f64>(*split.get(1)).map(|p| {
                        priorities.insert(c, p)
                    }));
                }
            },
            None => { from_str(*encoding).map(|c| priorities.insert(c, 1.0)); }
        }
    }
    Ok(priorities)
}

