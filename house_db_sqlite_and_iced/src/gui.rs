pub mod gui_runner {
    use crate::implement_db_trait::implement::DevInRoom;
    use crate::DbQueries;
    use crate::ErrorDb;
    use async_trait::async_trait;
    use console::{style, Term};
    use futures::executor::block_on;
    use iced::executor;
    use iced::theme::{self, Theme};
    use iced::widget::{button, checkbox, column, row, text, text_input};
    use iced::{Alignment, Application, Color, Command, Element, Length, Sandbox, Settings};
    use sqlx::SqlitePool;
    use std::fmt::Write;
    use std::sync::Arc;
    use tokio::time::{sleep, Duration};
    pub struct ShouseGUI {
        db: Arc<SqlitePool>,
        val: i32,
        textval: String,
        counts: usize,
        input_value: String,
    }

    impl ShouseGUI {
        fn newdbc(db: &Arc<SqlitePool>) -> Self {
            Self {
                db: Arc::clone(db),
                val: 0,
                counts: 0,
                textval: String::new(),
                input_value: String::new(),
            }
        }
    }
    #[derive(Debug, Clone)]
    pub enum Msg {
        One,
        Two,
        Asy,
        ButtonPressed,
        AsyResq(i32),
        AsyDB,
        AsyDbRes(String),
        CloseConn,
        InputChanged(String),
    }
    async fn testme() -> i32 {
        sleep(Duration::from_millis(100)).await;
        42424242
    }
    impl Application for ShouseGUI {
        type Executor = executor::Default;
        type Flags = Arc<SqlitePool>;
        type Message = Msg;
        type Theme = Theme;

        fn new(flags: Self::Flags) -> (Self, Command<Self::Message>) {
            (ShouseGUI::newdbc(&flags), Command::none())
        }

        fn title(&self) -> String {
            String::from("Smart House Gui example")
        }

        fn update(&mut self, _message: Self::Message) -> Command<Self::Message> {
            match _message {
                Msg::One => self.val += 1,
                Msg::Two => self.val -= 1,
                Msg::Asy => {
                    return Command::perform(testme(), move |resp| {
                        //self.val = resp;
                        println!("val {resp}");
                        Msg::AsyResq(resp)
                    });
                }
                Msg::AsyResq(r) => self.val = r,
                Msg::ButtonPressed => {}
                Msg::AsyDB => {
                    /*                    return Command::perform(
                        get_all_devices_in_house(Arc::clone(&self.db), "smarthouse#1"),
                        |resp| {
                            println!("{}", style("got asyn response from db!").yellow());
                            Msg::AsyDbRes(resp.unwrap())
                        },
                    );*/
                    //NOT WORKING
                    return Command::perform(
                        {
                            let x = Arc::clone(&self.db);
                            async move {
                                x.get_device_info("smarthouse#1", "someroom#2", "device4")
                                    .await
                            }
                        },
                        move |resp| {
                            println!("info: {resp:?}");
                            Msg::AsyDbRes(resp.unwrap())
                        },
                    );
                }
                Msg::AsyDbRes(resp) => self.textval = resp.to_owned(),
                Msg::CloseConn => self.counts = Arc::strong_count(&self.db),
                Msg::InputChanged(val) => self.textval = val,
            }
            Command::none()
        }

        fn view(&self) -> Element<Self::Message> {
            let add_one = button("Add One")
                .width(Length::Units(65))
                .height(Length::Units(65))
                .style(theme::Button::Positive)
                .padding(10)
                .on_press(Msg::One);
            let min_one = button("Delete One")
                .width(Length::Units(65))
                .height(Length::Units(65))
                .padding(10)
                .style(theme::Button::Destructive)
                .on_press(Msg::Two);
            let close_connection = button("Close connection")
                .width(Length::Units(65))
                .height(Length::Units(65))
                .padding(10)
                .style(theme::Button::Destructive)
                .on_press(Msg::CloseConn);

            let some_value = text(format!("value is {}", self.val))
                .size(50)
                .width(Length::Units(350));
            let test_async_assign = button("Assign async")
                .width(Length::Units(65))
                .height(Length::Units(65))
                .style(theme::Button::Primary)
                .padding(15)
                .on_press(Msg::Asy);
            let button1 = button("Submit").padding(10).on_press(Msg::ButtonPressed);
            let device_info_button = button("get device info")
                .width(Length::Units(65))
                .height(Length::Units(65))
                .style(theme::Button::Primary)
                .padding(15)
                .on_press(Msg::AsyDB);
            let dev_info = text(self.textval.to_owned())
                .size(13)
                .width(Length::Fill)
                .height(Length::Fill)
                .style(theme::Text::Color(Color::BLACK));
            let s_counts = text(self.counts.to_owned())
                .size(13)
                .width(Length::Fill)
                .height(Length::Fill)
                .style(theme::Text::Color(Color::BLACK));
            let text_input = text_input("devname:", &self.input_value, Msg::InputChanged)
                .padding(10)
                .size(50);
            column![
                add_one,
                min_one,
                text_input,
                row![test_async_assign, some_value],
                device_info_button,
                dev_info,
                close_connection,
                s_counts,
                button1
            ]
            .spacing(10)
            .align_items(Alignment::Start)
            .into()
            //             "Hello, world!".into()
        }
    }
}
