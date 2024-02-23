use iced::{executor};
use iced::theme;
use iced::{Application, Command, Element, Settings, Theme, Alignment, Length};
use iced::widget::{container, column, row, scrollable, text};
mod header;
mod navigation;

pub fn main() -> iced::Result {
    Saage::run(Settings::default())
}


// TODO: Implement Button Messages to change views
#[derive(Debug, Clone)]
enum Message {
    Testing(Option<String>)
}

struct Saage;


impl Application for Saage {
    type Executor = executor::Default;
    type Flags = ();
    type Message = Message;
    type Theme = Theme;

    fn new(_flags: ()) -> (Saage, Command<Self::Message>) {
        (Saage, Command::none())
    }

    fn title(&self) -> String {
        String::from("Saage")
    }

    fn update(&mut self, _message: Self::Message) -> Command<Self::Message> {
        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
        let header = header::Header::new(Some("Test Title".to_string()), Message::Testing);
        let navigation = navigation::Navigation::new(Some("Test Title".to_string()), Message::Testing);

        let navigation_menu = column!(
            header,
            navigation
        ).width(200).height(Length::Fill);


        let content_view = container(text("This container will be actual content on pages")).center_x().height(Length::Fill).width(Length::Fill).padding(10);


        row![navigation_menu, content_view].into()

    }
}