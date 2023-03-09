#[macro_use]
extern crate prettytable;
mod modules;
use figlet_rs::FIGfont;
use modules::external::rune;
use prettytable::{Cell, Row, Table};

fn main() {
    //   rune(a);
    // create table
    let mut table = Table::new();
    table.add_row(row!["ABC", "DEFG", "HIJKLMN"]);
    table.add_row(row!["foobar", "bar", "foo"]);
    let x = "example".to_string();
    let custom_font = FIGfont::from_file("resources/larry3d.flf").unwrap();
    let figure = custom_font.convert(x.as_str());
    assert!(figure.is_some());
    println!("{}", figure.unwrap());
    //
    let custom_font = FIGfont::from_file("resources/isometric1.flf").unwrap();
    let figure = custom_font.convert(x.as_str());
    assert!(figure.is_some());
    println!("{}", figure.unwrap());
    //
    let custom_font = FIGfont::from_file("resources/univers.flf").unwrap();
    let figure = custom_font.convert(x.as_str());
    assert!(figure.is_some());
    let a = figure.unwrap();
    println!("{}", a);

    table.printstd();
}

trait Formatter {
    fn format(&self, data: String) -> String;
}
