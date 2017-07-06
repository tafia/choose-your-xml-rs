extern crate xml5ever;

use std::default::Default;
use std::env;
use std::io::BufReader;
use std::fs::File;

use xml5ever::tendril::{
    ByteTendril,
    ReadExt
};
use xml5ever::tokenizer::{
    EmptyTag,
    EndTag,
    ShortTag,
    StartTag,
    Token,
    TokenSink,
    XmlTokenizer,
    XmlTokenizerOpts,
};

fn main() {
    let args: Vec<_> = env::args().collect();

    if args.len() != 2 {
        println!("Usage:\n\txml5ever input.xml");
        return;
    }

    let sink = TokenPrinter {
        depth: 0,
        text_buf: String::new(),
    };

    let file = File::open(&args[1]).unwrap();
    let mut file = BufReader::new(file);

    let mut input = ByteTendril::new();
    file.read_to_tendril(&mut input).unwrap();
    let input = input.try_reinterpret().unwrap();
    let mut tok = XmlTokenizer::new(sink, XmlTokenizerOpts {
        profile: true,
        exact_errors: true,
        .. Default::default()
    });
    tok.feed(input);
    tok.end();
}

#[derive(Clone)]
struct TokenPrinter {
    depth: usize,
    text_buf: String,
}

impl TokenPrinter {
    fn write_text(&mut self) {
        if !self.text_buf.is_empty() {
            indent(self.depth);
            println!("Text: {:?}", self.text_buf);
            self.text_buf.clear();
        }
    }
}

impl TokenSink for TokenPrinter {
    fn process_token(&mut self, token: Token) {
        match token {
            Token::TagToken(tag) => {
                self.write_text();

                match tag.kind {
                    StartTag => {
                        indent(self.depth);

                        match &tag.name.prefix {
                            &Some(ref prefix) => println!("Start: {}:{}", prefix, tag.name.local),
                            &None => println!("Start: {}", tag.name.local),
                        }
                        // TODO: find out how to get a namespace
                        self.depth += 1;
                    }
                    EndTag => {
                        self.depth -= 1;
                        indent(self.depth);
                        println!("End: {}", tag.name.local);
                    }
                    ShortTag => {
                        indent(self.depth);
                        println!("Short: {}", tag.name.local);
                    }
                    EmptyTag => {
                        indent(self.depth);
                        println!("Empty: {}", tag.name.local);
                    }
                }

                for attr in tag.attrs.iter() {
                    indent(self.depth);
                    match &attr.name.prefix {
                        &Some(ref prefix) => println!("{}:{}=\"{}\"", prefix, attr.name.local, attr.value),
                        &None => println!("{}=\"{}\"", attr.name.local, attr.value),
                    }
                }
            }
            Token::CharacterTokens(b) => {
                self.text_buf.push_str(&*b)
            }
            Token::NullCharacterToken => {}
            Token::PIToken(d) => {
                self.write_text();
                indent(self.depth);
                println!("Processing Instruction: {} {}", d.target, d.data);
            }
            Token::DoctypeToken(d) => {
                self.write_text();
                indent(self.depth);
                println!("Document Type: {:?} {:?} {:?}", d.name, d.public_id, d.system_id);
            }
            Token::CommentToken(b) => {
                self.write_text();
                indent(self.depth);
                println!("Comment: '{}'", b);
            }
            Token::ParseError(err) => {
                indent(self.depth);
                println!("ERROR: {}", err);
            }
            Token::EOFToken => {}
        }
    }
}

fn indent(depth: usize) {
    for _ in 0..depth {
        print!("  ");
    }
}
