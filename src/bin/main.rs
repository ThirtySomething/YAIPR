use yaipr::Parser;

fn main() {
    let mut parser = Parser::new("sample-file.ini");
    parser.parse();
    parser.print_info();
}
