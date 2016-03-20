use std::io::BufRead;

use serde_json::Value;
use serde_json::from_str as serde_from_str;

pub struct Reader<IN: BufRead> {
    input: IN
}

impl<IN: BufRead> Reader<IN> {

    pub fn new(input: IN) -> Reader<IN> {
        Reader { input: input }
    }

}

impl<IN: BufRead> Iterator for Reader<IN> {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        let mut s = String::new();
        match self.input.read_line(&mut s) {
            Err(_) => None,
            Ok(_) => Some(String::from(s)),
        }
    }

}

pub struct JsonObjectReader<IN: BufRead> {
    reader: Reader<IN>,
}

impl<IN: BufRead> JsonObjectReader<IN> {

    pub fn new(reader: Reader<IN>) -> JsonObjectReader<IN> {
        JsonObjectReader { reader: reader }
    }

}

impl<IN: BufRead> Iterator for JsonObjectReader<IN> {
    type Item = Value;

    fn next(&mut self) -> Option<Value> {
        self.reader.next().and_then(|s| serde_from_str(&s[..]).ok())
    }

}

