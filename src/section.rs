// use regex::Regex;

#[derive(Debug)]
pub struct Section {
    sectiondataraw: String,
    sectionname: String,
    entrylist: Vec<String>,
}

impl Section {
    pub fn new(sectiondata: &str) -> Self {
        return Self {
            sectiondataraw: sectiondata.to_owned(),
            sectionname: "".to_string(),
            entrylist: Vec::new(),
        };
    }
}
