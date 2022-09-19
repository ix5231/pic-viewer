use std::path::PathBuf;

use iced::{
    button, executor, Alignment, Application, Button, Column, Command, Element, Error, Image,
    Length, Row, Settings, Text,
};
use native_dialog::FileDialog;

#[derive(Debug, Clone)]
struct PicViewer {
    pic_path: Option<PathBuf>,
    explore_picture: button::State,
    next: button::State,
    prev: button::State,
}

#[derive(Debug, Clone)]
enum Message {
    OpenFile,
}

fn select_picture() -> Result<Option<PathBuf>, native_dialog::Error> {
    FileDialog::new()
        .add_filter("Pictures", &["jpg", "png", "bmp"])
        .show_open_single_file()
}

impl Application for PicViewer {
    type Message = Message;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            PicViewer {
                pic_path: Option::None,
                explore_picture: button::State::new(),
                next: button::State::new(),
                prev: button::State::new(),
            },
            Command::none(),
        )
    }

    fn view(&mut self) -> Element<Self::Message> {
        let top_buttons = Row::new()
            .push(
                Button::new(&mut self.explore_picture, Text::new("test"))
                    .on_press(Message::OpenFile),
            )
            .push(Button::new(&mut self.prev, Text::new("Prev")))
            .push(Button::new(&mut self.next, Text::new("Next")));

        let mut controls = Column::new()
            .padding(10)
            .align_items(Alignment::Center)
            .width(Length::Fill)
            .push(top_buttons);

        if let Some(pic_path) = &self.pic_path {
            controls = controls.push(Image::new(pic_path));
        }

        controls.into()
    }

    fn update(&mut self, message: Message) -> Command<Self::Message> {
        match message {
            Message::OpenFile => {
                let selected_pic = select_picture();
                if let Ok(pic_path) = selected_pic {
                    self.pic_path = pic_path;
                }
                Command::none()
            }
        }
    }

    fn title(&self) -> String {
        "pic-viewer".to_string()
    }
}

fn main() -> Result<(), Error> {
    let mut settings = Settings::default();
    settings.window.size = (640, 480);
    PicViewer::run(settings)
}
