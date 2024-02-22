use iced::executor;
use iced::widget::container::Appearance;
use iced::{Application, Command, Element, Settings, Theme, Alignment, Length, theme};
use iced::widget::{container, column, row, horizontal_space, scrollable, button, text};
pub fn main() -> iced::Result {
    Saage::run(Settings::default())
}

struct Saage;

impl Application for Saage {
    type Executor = executor::Default;
    type Flags = ();
    type Message = ();
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
        let header = container(
            row![
                horizontal_space(),
                text(self.title()),
                horizontal_space(),
            ]
            .padding(10)
            .align_items(Alignment::Center),
        )
        .style(|theme: &Theme| {
            let palette = theme.extended_palette();
        
            container::Appearance::default()
                .with_border(palette.background.strong.color, 1).with_background(palette.primary.weak.color)
        });
        
        let navigation = container(
            column![button("Search").width(Length::Fill), button("Tracking").width(Length::Fill), button("Settings").width(Length::Fill), button("About").width(Length::Fill)]
                .spacing(20)
                .padding(10)
                .width(200)
                .align_items(Alignment::Center),
        )
        .style(theme::Container::Box)
        .height(Length::Fill);
        
        let content = container(
            scrollable(
                column![
                    "Content!",
                    "The end"
                ]
                .spacing(40)
                .align_items(Alignment::Center)
                .width(Length::Fill),
            )
            .height(Length::Fill),
        )
        .padding(10);
        
        column![header, row![navigation, content]].into()
    }

}




