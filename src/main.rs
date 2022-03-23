use iced::{button, executor, scrollable, 
    Align, Application, Button, Command, Clipboard, Element, Font,Settings, HorizontalAlignment, Row, Scrollable, Text, VerticalAlignment};

const FZFONT: Font = Font::External {
    name: "方正字体",
    bytes: include_bytes!("source/FZFWZhuZGDLHJW.TTF"),
};

struct Rokit{
    server_button_state: button::State,
    client_button_state: button::State,
    scrollable_state: scrollable::State,
}

#[derive(Debug, Clone)]
enum RokitMessage {
    ServerButton,
    ClientButton,
}

impl Application for Rokit {
    type Executor = executor::Default;
    type Message = RokitMessage;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Rokit, Command<Self::Message>) {
        (
            Rokit{
                server_button_state: button::State::new(),
                client_button_state: button::State::new(),
                scrollable_state: scrollable::State::new(),
            }, 
            Command::none()
        )
    }

    fn title(&self) -> String {
        String::from("Rokit")
    }

    fn update(&mut self, message: Self::Message, _clipboard: &mut Clipboard) -> Command<Self::Message> {
        match message {
            RokitMessage::ServerButton => println!("切换到服务器页面"),
            RokitMessage::ClientButton => println!("切换到客户端页面"),
        }
        Command::none()
    }

    fn view(&mut self) -> Element<Self::Message> {
        let server_button_text = Text::new("服务器")
            .font(FZFONT)
            .vertical_alignment(VerticalAlignment::Center)
            .horizontal_alignment(HorizontalAlignment::Center);
        let server_button = Button::new(&mut self.server_button_state, server_button_text)
            .on_press(RokitMessage::ServerButton)
            .min_width(60)
            .min_height(20)
            .padding(5);
        let client_button_text = Text::new("客户端")
            .font(FZFONT)
            .vertical_alignment(VerticalAlignment::Center)
            .horizontal_alignment(HorizontalAlignment::Center);
        let client_button = Button::new(&mut self.client_button_state, client_button_text)
            .on_press(RokitMessage::ClientButton)
            .min_width(60)
            .min_height(20)
            .padding(5);

        let column = Row::new()
            .push(server_button)
            .push(client_button)
            .padding(16)
            .spacing(12)
            .align_items(Align::Start)
            .max_height(1000000)
            .max_width(1000000); 

        Scrollable::new(&mut self.scrollable_state)
            .push(column)
            .into()
    }
}

fn generate_setting() -> Settings<()> {
    let mut setting = Settings::default();
    setting.window.size.0 = 600;
    setting.window.size.1 = 500;
    setting.window.resizable = true;
    setting.window.min_size = Option::Some((600, 500));

    setting
}
pub fn main() -> iced::Result {
    Rokit::run(generate_setting())
}

