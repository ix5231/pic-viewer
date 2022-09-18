use std::path::PathBuf;

use iced::Column;

#[derive(Debug, Clone)]
struct PicViewer {
    pic_path: PathBuf,
}

enum Message {}

impl PicViewer {
    pub fn view(&self) -> Column<Message> {
        column![Image::new(view.pic_path),]
    }
}

fn main() {
    println!("Hello, world!");
}
