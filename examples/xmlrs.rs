extern crate xml;

use std::env;
use std::io::BufReader;
use std::fs::File;
use std::str::{self, Utf8Error};

use xml::reader::{EventReader, XmlEvent, Error as XmlError};
use xml::common::XmlVersion;
use xml::ParserConfig;

#[derive(Debug)]
enum Error {
    XmlError(XmlError),
    Utf8Error(Utf8Error),
}

macro_rules! from_error {
    ($err_type:ty, $err_name:ident) => (
        impl From<$err_type> for Error {
            fn from(value: $err_type) -> Error {
                Error::$err_name(value)
            }
        }
    )
}

from_error!(XmlError, XmlError);
from_error!(Utf8Error, Utf8Error);

type Result<T> = std::result::Result<T, Error>;

fn main() {
    let args: Vec<_> = env::args().collect();

    if args.len() != 2 {
        println!("Usage:\n\txmlrs input.xml");
        return;
    }

    if let Err(e) = parse(&args[1]) {
        println!("{:?}", e);
    }
}

fn parse(path: &str) -> Result<()> {
    let file = File::open(path).unwrap();
    let file = BufReader::new(file);

    let config = ParserConfig::new()
        .trim_whitespace(true)
        .ignore_comments(false)
        .coalesce_characters(false);

    let parser = EventReader::new_with_config(file, config);
    let mut depth = 0;
    for e in parser {
        match e? {
            XmlEvent::StartDocument { version, encoding, standalone } => {
                indent(depth);
                println!("Declaration");

                let ver = match version {
                    XmlVersion::Version10 => "1.0",
                    XmlVersion::Version11 => "1.1",
                };
                indent(depth);
                println!("  version=\"{}\"", ver);

                indent(depth);
                println!("  encoding=\"{}\"", encoding);

                if let Some(v) = standalone {
                    indent(depth);
                    println!("  standalone=\"{}\"", v);
                }
            }
            XmlEvent::StartElement { name, attributes, namespace } => {
                indent(depth);

                if let Some(prefix) = name.prefix {
                    if let Some(ns) = namespace.get(&prefix) {
                        println!("Start: {} (ns: {})", name.local_name, ns);
                    }
                } else {
                    println!("Start: {}", name.local_name);
                }

                for a in attributes {
                    indent(depth + 1);

                    if let Some(prefix) = a.name.prefix {
                        if let Some(ns) = namespace.get(&prefix) {
                            println!("  Attribute: {}:{}=\"{}\" (ns: {})",
                                prefix, a.name.local_name, a.value, ns);
                        }
                    } else {
                        println!("  Attribute: {}=\"{}\"", a.name.local_name, a.value);
                    }
                }

                depth += 1;
            }
            XmlEvent::EndElement { name } => {
                depth -= 1;
                indent(depth);
                println!("End: {}", name.local_name);
            }
            XmlEvent::ProcessingInstruction { name, .. } => {
                indent(depth);
                println!("Processing Instruction: {}", name);
            }
            XmlEvent::CData(text) => {
                indent(depth);
                println!("CDATA: {:?}", text);
            }
            XmlEvent::Comment(text) => {
                indent(depth);
                println!("Comment: '{}'", text);
            }
            XmlEvent::Characters(text) => {
                indent(depth);
                println!("Characters: {:?}", text);
            }
            XmlEvent::Whitespace(text) => {
                indent(depth);
                println!("Whitespace: {:?}", text);
            }
            XmlEvent::EndDocument => {
                break;
            }
        }
    }

    Ok(())
}

fn indent(depth: usize) {
    for _ in 0..depth {
        print!("  ");
    }
}
