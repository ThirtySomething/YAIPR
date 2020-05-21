use regex::Regex;
use std::fs;

mod section;
use section::Section;

#[derive(Debug)]
pub struct Parser {
    filename: String,
    sectionlist: Vec<Section>,
}

impl Parser {
    pub fn new(filename: &str) -> Self {
        return Self {
            filename: filename.to_owned(),
            sectionlist: Vec::new(),
        };
    }

    pub fn parse(&mut self) {
        let contents = self.getfilecontent(&self.filename);
        let re_section = Regex::new(r"(?ms)(?P<SEC_FULL>^\[[^\[]*)").unwrap();

        for cap in re_section.captures_iter(&contents) {
            let mut new_section = Section::new(&cap["SEC_FULL"]);
            self.sectionlist.push(new_section);
        }
    }

    pub fn print_info(&self) {
        println!("Filname [{}]", self.filename);
        for sec in &self.sectionlist {
            println!("Section: {:?}", sec);
        }
    }

    fn getfilecontent(&self, filename: &str) -> std::string::String {
        assert!(!filename.is_empty());
        let content = fs::read_to_string(filename).expect("Something went wrong reading the file.");
        return content;
    }
}
