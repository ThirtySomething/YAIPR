use regex::Regex;

mod section;
use crate::sectionlist::section::Section;

#[derive(Debug)]
pub struct SectionList {
    pub sectionlist: Vec<Section>,
}

impl SectionList {
    pub fn new() -> Self {
        return Self {
            sectionlist: Vec::new(),
        };
    }

    pub fn parse(&mut self, contents: String) {
        let re_section = Regex::new(r"(?ms)(?P<SEC_FULL>^\[[^\[]*)").unwrap();

        for cap in re_section.captures_iter(&contents) {
            let mut new_section = Section::new();
            new_section.parse(cap["SEC_FULL"].to_string());
            self.sectionlist.push(new_section);
        }
    }
}
