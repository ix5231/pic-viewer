use std::path::PathBuf;

use iced::{Alignment, Column, Element, Error, Image, Length, Sandbox, Settings};

#[derive(Debug, Clone)]
struct PicViewer {
    pic_path: PathBuf,
}

#[derive(Debug)]
enum Message {}

impl Sandbox for PicViewer {
    type Message = Message;

    fn new() -> Self {
        PicViewer {
            pic_path: PathBuf::from(r"./Lenna.bmp"),
        }
    }

    fn view(&mut self) -> Element<Message> {
        Column::new()
            .padding(10)
            .align_items(Alignment::Center)
            .width(Length::Fill)
            .push(Image::new(&self.pic_path))
            .into()
    }

    fn update(&mut self, _message: Message) {}

    fn title(&self) -> String {
        "pic-viewer".to_string()
    }
}

fn main() -> Result<(), Error> {
    let mut settings = Settings::default();
    settings.window.size = (640, 480);
    PicViewer::run(settings)
}
