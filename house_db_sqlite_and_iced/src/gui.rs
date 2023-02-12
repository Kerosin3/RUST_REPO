pub mod gui_runner {
    use crate::DbQueries;
    use async_trait::async_trait;
    use futures::executor::block_on;
    use iced::executor;
    use iced::widget::{button, checkbox, column, row, text};
    use iced::{Alignment, Application, Command, Element, Length, Sandbox, Settings, Theme};
    use sqlx::SqlitePool;
    use tokio::time::{sleep, Duration};
    pub struct ShouseGUI<'dblife> {
        db: &'dblife SqlitePool,
        val: i32,
    }

    impl<'dblife> ShouseGUI<'dblife> {
        fn newdbc(db: &'dblife SqlitePool) -> Self {
            Self { db: db, val: 0 }
        }
    }
    #[derive(Debug, Clone)]
    pub enum Msg {
        One,
        Two,
        Asy,
        AsyResq(i32),
    }
    async fn testme() -> i32 {
        sleep(Duration::from_millis(100)).await;
        42424242
    }

    impl<'dblife> Application for ShouseGUI<'dblife> {
        type Executor = executor::Default;
        type Flags = &'dblife SqlitePool;
        type Message = Msg;
        type Theme = Theme;

        fn new(flags: Self::Flags) -> (Self, Command<Self::Message>) {
            (ShouseGUI::newdbc(flags), Command::none())
        }

        fn title(&self) -> String {
            String::from("A cool application")
        }

        fn update(&mut self, _message: Self::Message) -> Command<Self::Message> {
            match _message {
                Msg::One => self.val += 1,
                Msg::Two => self.val -= 1,
                Msg::Asy => {
                    return Command::perform(testme(), |resp| {
                        //self.val = resp;
                        println!("val {resp}");
                        Msg::AsyResq(resp)
                    });
                }
                Msg::AsyResq(r) => self.val = r,
            }
            Command::none()
        }

        fn view(&self) -> Element<Self::Message> {
            row![
                button("One")
                    .width(Length::Units(65))
                    .height(Length::Units(65))
                    .on_press(Msg::One),
                text(self.val).size(50),
                button("Two").on_press(Msg::Two),
                button("Asy").on_press(Msg::Asy),
            ]
            .padding(20)
            .align_items(Alignment::Center)
            .into()
            //             "Hello, world!".into()
        }
    }
}
