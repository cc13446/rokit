use iced::{executor, Application, Command, Element, Settings, Text, Clipboard};
struct Rokit;

impl Application for Rokit {
    type Executor = executor::Default;
    type Message = ();
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Rokit, Command<Self::Message>) {
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

fn generate_setting() -> Settings<()> {
    let mut setting = Settings::default();
    setting.window.size.0 = 400;
    setting.window.size.1 = 600;
    
    setting
}
pub fn main() -> iced::Result {
    Rokit::run(generate_setting())
}

