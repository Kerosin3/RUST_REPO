pub mod gui_runner {
    use crate::implement_db_trait::implement::DevInRoom;
    use crate::DbQueries;
    use crate::ErrorDb;
    use async_trait::async_trait;
    use console::{style, Term};
    use futures::executor::block_on;
    use iced::executor;
    use iced::theme::{self, Theme};
    use iced::widget::{button, checkbox, column, row, text};
    use iced::{Alignment, Application, Color, Command, Element, Length, Sandbox, Settings};
    use sqlx::SqlitePool;
    use std::fmt::Write;
    use std::sync::Arc;
    use tokio::time::{sleep, Duration};
    pub struct ShouseGUI {
        db: Arc<SqlitePool>,
        val: i32,
        textval: String,
    }

    impl ShouseGUI {
        fn newdbc(db: Arc<SqlitePool>) -> Self {
            Self {
                db: Arc::clone(&db),
                val: 0,
                textval: String::new(),
            }
        }
    }
    #[derive(Debug, Clone)]
    pub enum Msg {
        One,
        Two,
        Asy,
        AsyResq(i32),
        AsyDB,
        AsyDbRes(String),
    }
    async fn testme() -> i32 {
        sleep(Duration::from_millis(100)).await;
        42424242
    }
    //it works!
    async fn get_all_devices_in_house(
        db: Arc<SqlitePool>,
        housename: &str,
    ) -> Result<String, ErrorDb> {
        match sqlx::query_as::<_, DevInRoom>(
                "SELECT devid, devname, attached_to_house, attached_to_room, info, active, timestamp FROM devices"
            )
            .fetch_all(&*db) // ?????
            .await
            {
                Ok(_results) => {
                    let mut out = String::new();
                    writeln!(out, "---info about all devices in house {housename} ---")
                        .expect("error while writing to string");
                    for r in _results {
                        writeln!(
                            out,
                            "dev id [{}], dev name: {} room: {}, house: {} active:{} info: {}, data creation: {}",
                            r.devid, &r.devname, &r.attached_to_room, &r.attached_to_house, r.active, &r.info,&r.timestamp
                        )
                        .expect("error while writing to string");
                    }
//                     println!("{}", style(&out).green());
                    Ok(out)
                }
                Err(e) => {
                    tracing::error!("error:{e} while getting info all devices in house");
                    Err(ErrorDb::ErrorQuery(
                        "error getiing info about all devices".to_owned(),
                    ))
                }
            }
    }

    impl Application for ShouseGUI {
        type Executor = executor::Default;
        type Flags = Arc<SqlitePool>;
        type Message = Msg;
        type Theme = Theme;

        fn new(flags: Self::Flags) -> (Self, Command<Self::Message>) {
            (ShouseGUI::newdbc(flags), Command::none())
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
                        Arc::clone(&self.db).get_device_info(
                            "smarthouse#1",
                            "someroom#2",
                            "device4",
                        ),
                        |resp| {
                            println!("info: {resp:?}");
                            Msg::AsyDbRes(resp.unwrap())
                        },
                    );
                }
                Msg::AsyDbRes(resp) => self.textval = resp.to_owned(),
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
            let some_value = text(format!("value is {}", self.val))
                .size(50)
                .width(Length::Units(350));
            let test_async_assign = button("Assign async")
                .width(Length::Units(65))
                .height(Length::Units(65))
                .style(theme::Button::Primary)
                .padding(15)
                .on_press(Msg::Asy);

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
            column![
                add_one,
                min_one,
                row![test_async_assign, some_value],
                device_info_button,
                dev_info
            ]
            .spacing(10)
            .align_items(Alignment::Start)
            .into()
            //             "Hello, world!".into()
        }
    }
}
