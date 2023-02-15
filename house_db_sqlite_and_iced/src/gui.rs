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

    #[derive(Debug, Clone, Default)]
    struct SearchStruct {
        dev: String,
        room: String,
        house: String,
    }
    pub struct ShouseGUI {
        db: Arc<SqlitePool>,
        val: i32,
        textval: String,
        counts: usize,
        search: SearchStruct,
    }

    impl ShouseGUI {
        fn newdbc(db: &Arc<SqlitePool>) -> Self {
            Self {
                db: Arc::clone(db),
                val: 0,
                counts: 0,
                textval: String::new(),
                search: Default::default(),
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
        InputChanged(SearchStruct),
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
                    return Command::perform(
                        {
                            let x = Arc::clone(&self.db);
                            let dev = self.search.dev.to_owned();
                            let room = self.search.room.to_owned();
                            let house = self.search.house.to_owned();
                            async move {
                                x.get_device_info("smarthouse#1", "someroom#2", &dev)
                                    //                                 x.get_device_info("smarthouse#1", "someroom#2", "device4")
                                    .await
                            }
                        },
                        move |resp| {
                            println!("info: {resp:?}");
                            match resp {
                                Ok(r) => return Msg::AsyDbRes(r),
                                Err(_e) => return Msg::AsyDbRes("Not such device".to_owned()),
                            }
                            Msg::AsyDbRes(resp.unwrap())
                        },
                    );
                }
                Msg::AsyDbRes(resp) => self.textval = resp.to_owned(),
                Msg::CloseConn => self.counts = Arc::strong_count(&self.db),
                Msg::InputChanged(val) =>{ self.search.dev = val,
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

            let some_value = text(format!("value is :{}", self.val))
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
                .width(Length::Units(100))
                .height(Length::Units(65))
                .style(theme::Button::Primary)
                .padding(15)
                .on_press(Msg::AsyDB);
            let dev_search = text(format!("dev to search: {}", self.search.dev.to_owned()))
                .size(50)
                .width(Length::Fill)
                .height(Length::Fill)
                .style(theme::Text::Color(Color::BLACK));
            let room_search = text(format!("room to search: {}", self.search.room.to_owned()))
                .size(50)
                .width(Length::Fill)
                .height(Length::Fill)
                .style(theme::Text::Color(Color::BLACK));
            let house_search = text(format!("house to search: {}", self.search.house.to_owned()))
                .size(50)
                .width(Length::Fill)
                .height(Length::Fill)
                .style(theme::Text::Color(Color::BLACK));

            let dev_info = text(self.textval.to_owned())
                .size(50)
                .width(Length::Fill)
                .height(Length::Fill)
                .style(theme::Text::Color(Color::BLACK));
            let s_counts = text(self.counts.to_owned())
                .size(13)
                .width(Length::Fill)
                .height(Length::Fill)
                .style(theme::Text::Color(Color::BLACK));
            let input_size = 30_u16;
            let dev_input = text_input("devname:", &self.search, Msg::InputChanged(dev))
                .padding(10)
                .size(input_size);
            let house_input = text_input("house:", &self.search.house, Msg::InputChanged)
                .padding(10)
                .size(input_size);
            let room_input = text_input("room:", &self.search.room, Msg::InputChanged)
                .padding(10)
                .size(input_size);

            column![
                add_one,
                min_one,
                row![test_async_assign, some_value],
                row![dev_input, house_input, room_input],
                dev_info,
                device_info_button,
                dev_search,
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
