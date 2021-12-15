use std::{result::Result};

mod tests;
pub mod document;
mod tree;
mod tokenizer;
mod parser;
mod states;

pub struct RawDocument {
    raw : String,
    _encoding : EncodingConfidence,
}

pub enum EncodingConfidence {
    Tentative(Encoding),
    Certain(Encoding),
    Irrelevant
}

//TODO : Add more encodings from the spec
pub enum Encoding {
    Utf8,
}

impl RawDocument {
    pub fn new(document : &str) -> Result<Self, Box<dyn std::error::Error>> {
        RawDocument::preprocess(document)
    }
    pub fn append_and_revalidate(&mut self, value : &str) -> Result<(), Box<dyn std::error::Error>> {
        self.raw.push_str(value);
        if let Err(e) = RawDocument::preprocess(&self.raw) {
            self.raw.truncate(value.len());
            Err(e)
        } else {
            Ok(())
        }
    }
    fn preprocess(doc : &str) -> Result<RawDocument, Box<dyn std::error::Error>> {
        // TODO : Do pre-scan of document.
        // https://dev.w3.org/html5/spec-LC/parsing.html
        // - [ ] Determine the encoding of the document.
        // - [ ] Change all foreign encoded characters to utf-8
        let doc = Self {
            raw : String::from(doc),
            _encoding : EncodingConfidence::Irrelevant,
        };
        Ok(doc)
    }
}