use regex::Regex;

#[derive(Debug)]
pub struct Section {
    sectionname: String,
    entrylist: Vec<String>,
}

impl Section {
    pub fn new() -> Self {
        return Self {
            sectionname: "".to_string(),
            entrylist: Vec::new(),
        };
    }

    pub fn parse(&mut self, contents: String) {
        let re_section = Regex::new(r"(?ms)(^\[(?P<SEC_NAME>.*)\])").unwrap();
        let caps = re_section.captures(&contents).unwrap();
        self.sectionname = caps[1].to_string();
    }
}
