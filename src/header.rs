use iced::alignment::Alignment;
use iced::{Length, Theme};
use iced::widget::{component, row, text, horizontal_space, container, Component};
use iced::{Element};


pub struct Header<Message> {
    content_title: Option<String>,
    on_change: Box<dyn Fn(Option<String>) -> Message>,
}

impl<Message> Header<Message> {
    pub fn new(content_title: Option<String>, on_change: impl Fn(Option<String>) -> Message + 'static,) -> Self {
        return Self {content_title, on_change: Box::new(on_change)};
    }

    pub fn set_title(&mut self, new_title : Option<String>) {
        self.content_title = new_title;
    }
}

impl<Message> Component<Message> for Header<Message> {
    type State = ();
    type Event = ();

    fn update(&mut self, state: &mut Self::State, event: Self::Event,) -> Option<Message> {
        todo!()
    }

    fn view(&self, state: &Self::State,) -> Element<Self::Event> {
        container(
            row![
                horizontal_space(),
                text(self.content_title.clone().unwrap()),
                horizontal_space(),
            ]
            .padding(10)
            .align_items(Alignment::Center),
        )
        .style(|theme: &Theme| {
            let palette = theme.extended_palette();
        
            container::Appearance::default()
                .with_border(palette.background.strong.color, 1).with_background(palette.primary.weak.color)
        })
        .into()
    }
}


impl<'a, Message> From<Header<Message>> for Element<'a, Message> where Message: 'a,
{
    fn from(header: Header<Message>) -> Self {
        component(header)
    }
}