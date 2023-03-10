mod modules;
use figlet_rs::FIGfont;
// rust strategy pattern example
fn main() {
    let x = String::from("example"); // create example string
                                     // use default font formatter
    let report1 = Report::generate(DefaultFont, x.to_owned());
    println!("{}", report1);
    // use custom font 1 formatter realization
    let report2 = Report::generate(modules::mod2::implementv1::CustomFont1, x.to_owned());
    println!("{}", report2);
    // use custom font 2 formatter realization
    let report3 = Report::generate(modules::mod1::implementv1::CustomFont2, x);
    println!("{}", report3);
}

// declare public api
pub trait Formatter {
    fn format(&self, data: String) -> String;
}
struct Report;
impl Report {
    pub fn generate<T: Formatter>(fmt: T, data: String) -> String {
        fmt.format(data)
    }
}
// default font
struct DefaultFont;
impl Formatter for DefaultFont {
    fn format(&self, data: String) -> String {
        let custom_font = FIGfont::from_file("resources/larry3d.flf")
            .expect("please run cargo from this example root folder");
        let figure = custom_font.convert(data.as_str());

        format!("{}", figure.expect("error painting figure"))
    }
}
