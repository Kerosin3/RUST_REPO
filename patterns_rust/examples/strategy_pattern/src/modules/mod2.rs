pub mod implementv1 {
    static HEADING: &str = "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx";
    static ENDING: &str = "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx";
    use crate::Formatter;
    use figlet_rs::FIGfont;
    pub struct CustomFont1;
    use std::fmt;
    impl Formatter for CustomFont1 {
        fn format(&self, data: String) -> String {
            let custom_font = FIGfont::from_file("resources/isometric1.flf")
                .expect("please run cargo from this example root folder");
            let figure = custom_font.convert(data.as_str());
            let mut output = String::new();
            fmt::write(
                &mut output,
                format_args!(
                    "{}\n{}\n{}",
                    HEADING,
                    figure.expect("error painting figure"),
                    ENDING
                ),
            )
            .expect("Error occurred while trying to write in String");
            output
        }
    }
}
