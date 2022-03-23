use iced::{executor, Application, Command, Element, Settings, Text, Clipboard};

pub fn main() -> iced::Result {
    Rokit::run(Settings::default())
}

struct Rokit;

impl Application for Rokit {
    type Executor = executor::Default;
    type Message = ();
    type Flags = ();

    fn new(_flags: ()) -> (Rokit, Command<Self::Message>) {
        (Rokit, Command::none())
    }

    fn title(&self) -> String {
        String::from("Rokit")
    }

    fn update(&mut self, _message: Self::Message, _clipboard: &mut Clipboard) -> Command<Self::Message> {
        Command::none()
    }

    fn view(&mut self) -> Element<Self::Message> {
        Text::new("Hello, world!").into()
    }
}