use iced::alignment::Alignment;
use iced::Theme;
use iced::theme;
use iced::widget::{component, button, row, text, horizontal_space, column, container, Component};
use iced::{Element, Length};


// TODO: Implement Button Switching
pub struct Navigation<Message> {
    content_title: Option<String>,
    on_change: Box<dyn Fn(Option<String>) -> Message>,
}

impl<Message> Navigation<Message> {
    pub fn new(content_title: Option<String>, on_change: impl Fn(Option<String>) -> Message + 'static,) -> Self {
        return Self {content_title, on_change: Box::new(on_change)};
    }

    pub fn set_title(&mut self, new_title : Option<String>) {
        self.content_title = new_title;
    }
}

impl<Message> Component<Message> for Navigation<Message> {
    type State = ();
    type Event = ();

    fn update(&mut self, state: &mut Self::State, event: Self::Event,) -> Option<Message> {
        todo!()
    }

    fn view(&self, state: &Self::State,) -> Element<Self::Event> {
        container(
            column![button("Search").width(Length::Fill), button("Tracking").width(Length::Fill), button("Settings").width(Length::Fill), button("About").width(Length::Fill)]
                .spacing(20)
                .padding(10)
                .align_items(Alignment::Center),
        )
        .style(theme::Container::Box)
        .height(Length::Fill)
        .into()
    }
}


impl<'a, Message> From<Navigation<Message>> for Element<'a, Message> where Message: 'a,
{
    fn from(navigation: Navigation<Message>) -> Self {
        component(navigation)
    }
}