#![allow(unreachable_code)]

use std::path::Path;
use std::io::{BufReader, Read};
use std::fs::File;
use std::env;

use xml::reader::XmlEvent;
use xml::reader::ParserConfig;
use xml::EventReader;

fn main() -> std::io::Result<()> {
    env_logger::init();
    let args = env::args().skip(1).collect::<Vec<String>>();
    log::info!("args = {:?}", args);
    if args.is_empty() {
        log::info!("You must input xml file!");
        return std::process::exit(-1);
    }

    let path = &args[0];
    log::info!("path = {}", path);

    let file = File::open(path)?;
    let file = BufReader::new(file);

    let parser : EventReader<BufReader<File>> = ParserConfig::new()
        .trim_whitespace(true)
        .ignore_comments(true)
        .coalesce_characters(true)
        .create_reader(file);
    
    let mut depth = 0;
    for e in parser {
        match e {
            Ok(XmlEvent::StartElement { name, ..}) => {
                log::info!("{}+{}", indent(depth), name);
                depth += 1;
            }
            Ok(XmlEvent::Characters(content)) => {
                depth += 1;
                log::info!("{}->{}", indent(depth), content);
                depth -= 1;
            }
            Ok(XmlEvent::EndElement { name }) => {
                depth -= 1;
                log::info!("{}-{}", indent(depth), name);
            }
            Err(e) => {
                log::info!("Error: {}", e);
                break;
            }
            _ => {}
        }
    }

    Ok(())
}

fn indent(size: usize) -> String {
    const INDENT: &'static str = "    ";
    (0..size).map(|_| INDENT)
        .fold(String::with_capacity(size*INDENT.len()), |r, s| r + s)
}
