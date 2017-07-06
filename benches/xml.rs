#[macro_use]
extern crate bencher;

extern crate xml;
extern crate quick_xml;
extern crate xml5ever;
extern crate sxd_document;

use std::fs;
use std::env;
use std::io::Read;

use bencher::Bencher;

fn load_file(path: &str) -> String {
    let path = env::current_dir().unwrap().join(path);
    let mut file = fs::File::open(&path).unwrap();
    let mut text = String::new();
    file.read_to_string(&mut text).unwrap();
    text
}

fn xmlrs_parse(text: &str) {
    for event in xml::EventReader::new(text.as_bytes()) {
        let _ = event.unwrap();
    }
}

fn xmlrs_small(bencher: &mut Bencher) {
    let text = load_file("data/small.xml");
    bencher.iter(|| xmlrs_parse(&text))
}

fn xmlrs_medium(bencher: &mut Bencher) {
    let text = load_file("data/medium.xml");
    bencher.iter(|| xmlrs_parse(&text))
}

fn xmlrs_large(bencher: &mut Bencher) {
    let text = load_file("data/large.plist");
    bencher.iter(|| xmlrs_parse(&text))
}

fn quick_xml_parse(text: &str) {
    let mut t = quick_xml::reader::Reader::from_reader(text.as_bytes());
    t.check_comments(true);
    let mut buf = Vec::new();
    let mut ns_buf = Vec::new();
    loop {
        match t.read_namespaced_event(&mut buf, &mut ns_buf) {
            Ok((_, quick_xml::events::Event::Start(ref e))) => {
                for a in e.attributes() {
                    let _ = a.unwrap().unescaped_value();
                }
            }
            Ok((_, quick_xml::events::Event::Text(ref e))) => { let _ = e.unescaped(); }
            Ok((_, quick_xml::events::Event::Eof)) => break,
            _ => {}
        }
    }
}

fn quick_xml_small(bencher: &mut Bencher) {
    let text = load_file("data/small.xml");
    bencher.iter(|| quick_xml_parse(&text))
}

fn quick_xml_medium(bencher: &mut Bencher) {
    let text = load_file("data/medium.xml");
    bencher.iter(|| quick_xml_parse(&text))
}

fn quick_xml_large(bencher: &mut Bencher) {
    let text = load_file("data/large.plist");
    bencher.iter(|| quick_xml_parse(&text))
}

struct Xml5Token;

impl xml5ever::tokenizer::TokenSink for Xml5Token {
    fn process_token(&mut self, _: xml5ever::tokenizer::Token) { }
}

fn xml5ever_parse(text: &str) {
    let sink = Xml5Token;

    let input = xml5ever::tendril::StrTendril::from_slice(text);

    let mut t = xml5ever::tokenizer::XmlTokenizer::new(sink, Default::default());
    t.feed(input);
    t.end();
}

fn xml5ever_small(bencher: &mut Bencher) {
    let text = load_file("data/small.xml");
    bencher.iter(|| xml5ever_parse(&text))
}

fn xml5ever_medium(bencher: &mut Bencher) {
    let text = load_file("data/medium.xml");
    bencher.iter(|| xml5ever_parse(&text))
}

fn xml5ever_large(bencher: &mut Bencher) {
    let text = load_file("data/large.plist");
    bencher.iter(|| xml5ever_parse(&text))
}

fn sxd_document_parse(text: &str) {
    let _ = sxd_document::parser::parse(&text).unwrap();
}

fn sxd_document_small(bencher: &mut Bencher) {
    let text = load_file("data/small.xml");
    bencher.iter(|| sxd_document_parse(&text))
}

fn sxd_document_medium(bencher: &mut Bencher) {
    let text = load_file("data/medium.xml");
    bencher.iter(|| sxd_document_parse(&text))
}

// sxd_document can't open large.plist

benchmark_group!(benches1, xmlrs_small, xmlrs_medium, xmlrs_large);
benchmark_group!(benches2, quick_xml_small, quick_xml_medium, quick_xml_large);
benchmark_group!(benches3, xml5ever_small, xml5ever_medium, xml5ever_large);
benchmark_group!(benches4, sxd_document_small, sxd_document_medium);
benchmark_main!(benches1, benches2, benches3, benches4);
