use std::fs;

mod sectionlist;
use crate::sectionlist::SectionList;

#[derive(Debug)]
pub struct Parser {
    filename: String,
    sectionlist: SectionList,
}

impl Parser {
    pub fn new(filename: &str) -> Self {
        return Self {
            filename: filename.to_owned(),
            sectionlist: SectionList::new(),
        };
    }

    pub fn parse(&mut self) {
        let contents = self.getfilecontent(&self.filename);
        self.sectionlist.parse(contents);
    }

    pub fn print_info(&self) {
        println!("Filname [{}]", self.filename);
        for sec in &self.sectionlist.sectionlist {
            println!("Section: {:?}", sec);
        }
    }

    fn getfilecontent(&self, filename: &str) -> std::string::String {
        assert!(!filename.is_empty());
        let content = fs::read_to_string(filename).expect("Something went wrong reading the file.");
        return content;
    }
}
