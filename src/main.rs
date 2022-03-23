use iced::{button, executor, scrollable, text_input,
    Align, Application, Button, Command, Column, Clipboard, Element, Font, Settings, HorizontalAlignment, 
    Length, Row, Scrollable, Text, TextInput, VerticalAlignment};

const FZFONT: Font = Font::External {
    name: "方正字体",
    bytes: include_bytes!("source/FZFWZhuZGDLHJW.TTF"),
};

const SERVER_TCP_BUTTON_TEXT_CONNECT : &str = "TCP服务器启动";
const SERVER_TCP_BUTTON_TEXT_DISCONNECT : &str = "TCP服务器停止";
const SERVER_UDP_BUTTON_TEXT_CONNECT : &str = "UDP服务器启动";
const SERVER_UDP_BUTTON_TEXT_DISCONNECT : &str = "UDP服务器停止";
const CLIENT_TCP_BUTTON_TEXT_CONNECT : &str = "TCP连接";
const CLIENT_TCP_BUTTON_TEXT_DISCONNECT : &str = "TCP断开";
const CLIENT_UDP_BUTTON_TEXT_CONNECT : &str = "UDP连接";
const CLIENT_UDP_BUTTON_TEXT_DISCONNECT : &str = "UDP断开";

struct Rokit{

    server_tcp_ip_text_input_state: text_input::State,
    server_tcp_ip_text_input: String,
    server_tcp_port_text_input_state: text_input::State,
    server_tcp_port_text_input: String,
    server_tcp_button_text:String,
    server_tcp_button_state: button::State,

    server_udp_ip_text_input_state: text_input::State,
    server_udp_ip_text_input: String,
    server_udp_port_text_input_state: text_input::State,
    server_udp_port_text_input: String,
    server_udp_button_text:String,
    server_udp_button_state: button::State,

    client_ip_text_input_state: text_input::State,
    client_ip_text_input: String,
    client_port_text_input_state: text_input::State,
    client_port_text_input: String,
    client_tcp_button_text:String,
    client_tcp_button_state: button::State,

    client_udp_button_text:String,
    client_udp_button_state: button::State,

    scrollable_state:scrollable::State,
}

#[derive(Debug, Clone)]
enum RokitMessage {
    ServerTCPIPTextInput(String),
    ServerTCPPortTextInput(String),
    ServerTCPButton,

    ServerUDPIPTextInput(String),
    ServerUDPPortTextInput(String),
    ServerUDPButton,

    ClientIPTextInput(String),
    ClientPortTextInput(String),
    ClientTCPButton,
    ClientUDPButton,
    
}

impl Application for Rokit {
    type Executor = executor::Default;
    type Message = RokitMessage;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Rokit, Command<Self::Message>) {
        (
            Rokit{
                server_tcp_ip_text_input_state : text_input::State::new(),
                server_tcp_ip_text_input:String::from("127.0.0.1"),
                server_tcp_port_text_input_state : text_input::State::new(),
                server_tcp_port_text_input:String::from("8888"),
                server_tcp_button_text: String::from(SERVER_TCP_BUTTON_TEXT_CONNECT),
                server_tcp_button_state: button::State::new(),

                server_udp_ip_text_input_state : text_input::State::new(),
                server_udp_ip_text_input:String::from("127.0.0.1"),
                server_udp_port_text_input_state : text_input::State::new(),
                server_udp_port_text_input:String::from("8888"),
                server_udp_button_text: String::from(SERVER_UDP_BUTTON_TEXT_CONNECT),
                server_udp_button_state: button::State::new(),
                
                client_ip_text_input_state: text_input::State::new(),
                client_ip_text_input: String::from("127.0.0.1"),
                client_port_text_input_state: text_input::State::new(),
                client_port_text_input: String::from("8888"),
                client_tcp_button_text:String::from(CLIENT_TCP_BUTTON_TEXT_CONNECT),
                client_tcp_button_state: button::State::new(),

                client_udp_button_text:String::from(CLIENT_UDP_BUTTON_TEXT_CONNECT),
                client_udp_button_state: button::State::new(),
               
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
            RokitMessage::ServerTCPIPTextInput(s) => {
                self.server_tcp_ip_text_input = s;
            },
            RokitMessage::ServerTCPPortTextInput(s) => {
                self.server_tcp_port_text_input = s;
            },
            RokitMessage::ServerTCPButton => {
                self.server_tcp_button_text = String::from(SERVER_TCP_BUTTON_TEXT_DISCONNECT)
            },
            RokitMessage::ServerUDPIPTextInput(s) => {
                self.server_udp_ip_text_input = s;
            },
            RokitMessage::ServerUDPPortTextInput(s) => {
                self.server_udp_port_text_input = s;
            },
            RokitMessage::ServerUDPButton => {
                self.server_udp_button_text = String::from(SERVER_UDP_BUTTON_TEXT_DISCONNECT)
            },

            RokitMessage::ClientIPTextInput(s) => {
                self.client_ip_text_input = s;
            },
            RokitMessage::ClientPortTextInput(s) => {
                self.client_port_text_input = s;
            },
            RokitMessage::ClientTCPButton => {
                self.client_tcp_button_text = String::from(CLIENT_TCP_BUTTON_TEXT_DISCONNECT)
            },
            RokitMessage::ClientUDPButton => {
                self.client_udp_button_text = String::from(CLIENT_UDP_BUTTON_TEXT_DISCONNECT)
            },
            
        }
        Command::none()
    }

    fn view(&mut self) -> Element<Self::Message> {
        let server_text = Text::new("Socket服务器")
            .font(FZFONT)
            .vertical_alignment(VerticalAlignment::Center)
            .horizontal_alignment(HorizontalAlignment::Center);
        
        let server_tcp_ip_text_input = TextInput::new(&mut self.server_tcp_ip_text_input_state, "IP地址",&self.server_tcp_ip_text_input,RokitMessage::ServerTCPIPTextInput)
            .font(FZFONT)
            .width(Length::FillPortion(1)) 
            .padding(5);
        let server_tcp_port_text_input = TextInput::new(&mut self.server_tcp_port_text_input_state, "端口",&self.server_tcp_port_text_input,RokitMessage::ServerTCPPortTextInput)
            .font(FZFONT)
            .width(Length::FillPortion(1)) 
            .padding(5);
        let server_tcp_button_text = Text::new(&self.server_tcp_button_text)
            .font(FZFONT)
            .size(17)
            .vertical_alignment(VerticalAlignment::Center)
            .horizontal_alignment(HorizontalAlignment::Center);
        let server_tcp_button = Button::new(&mut self.server_tcp_button_state, server_tcp_button_text)
            .on_press(RokitMessage::ServerTCPButton)
            .width(Length::FillPortion(1))
            .padding(5);
        let server_tcp_row = Row::new()
            .push(server_tcp_ip_text_input)
            .push(server_tcp_port_text_input)
            .push(server_tcp_button)
            .padding(0)
            .spacing(2)
            .align_items(Align::Center)
            .max_height(1000000)
            .max_width(1000000);

        let server_udp_ip_text_input = TextInput::new(&mut self.server_udp_ip_text_input_state, "IP地址",&self.server_udp_ip_text_input,RokitMessage::ServerUDPIPTextInput)
            .font(FZFONT)
            .width(Length::FillPortion(1)) 
            .padding(5);
        let server_udp_port_text_input = TextInput::new(&mut self.server_udp_port_text_input_state, "端口",&self.server_udp_port_text_input,RokitMessage::ServerUDPPortTextInput)
            .font(FZFONT)
            .width(Length::FillPortion(1)) 
            .padding(5);
        let server_udp_button_text = Text::new(&self.server_udp_button_text)
            .font(FZFONT)
            .size(17)
            .vertical_alignment(VerticalAlignment::Center)
            .horizontal_alignment(HorizontalAlignment::Center);
        let server_udp_button = Button::new(&mut self.server_udp_button_state, server_udp_button_text)
            .on_press(RokitMessage::ServerUDPButton)
            .width(Length::FillPortion(1))
            .padding(5);
        let server_udp_row = Row::new()
            .push(server_udp_ip_text_input)
            .push(server_udp_port_text_input)
            .push(server_udp_button)
            .padding(0)
            .spacing(2)
            .align_items(Align::Center)
            .max_height(1000000)
            .max_width(1000000);
        
        let server_column = Column::new()
            .push(server_text)
            .push(server_tcp_row)
            .push(server_udp_row)
            .padding(16)
            .spacing(12)
            .align_items(Align::Start)
            .width(Length::FillPortion(1))
            .max_height(1000000)
            .max_width(1000000); 

        let client_text = Text::new("Socket客户端")
            .font(FZFONT)
            .vertical_alignment(VerticalAlignment::Center)
            .horizontal_alignment(HorizontalAlignment::Center);

        let client_ip_text_input = TextInput::new(&mut self.client_ip_text_input_state, "IP地址",&self.client_ip_text_input,RokitMessage::ClientIPTextInput)
            .font(FZFONT)
            .width(Length::FillPortion(1)) 
            .padding(5);
        let client_port_text_input = TextInput::new(&mut self.client_port_text_input_state, "端口",&self.client_port_text_input,RokitMessage::ClientPortTextInput)
            .font(FZFONT)
            .width(Length::FillPortion(1)) 
            .padding(5);
        let client_tcp_button_text = Text::new(&self.client_tcp_button_text)
            .font(FZFONT)
            .size(15)
            .vertical_alignment(VerticalAlignment::Center)
            .horizontal_alignment(HorizontalAlignment::Center);
        let client_tcp_button = Button::new(&mut self.client_tcp_button_state, client_tcp_button_text)
            .on_press(RokitMessage::ClientTCPButton)
            .width(Length::FillPortion(1))
            .padding(5);
        let client_udp_button_text = Text::new(&self.client_udp_button_text)
            .font(FZFONT)
            .size(15)
            .vertical_alignment(VerticalAlignment::Center)
            .horizontal_alignment(HorizontalAlignment::Center);
        let client_udp_button = Button::new(&mut self.client_udp_button_state, client_udp_button_text)
            .on_press(RokitMessage::ClientUDPButton)
            .width(Length::FillPortion(1))
            .padding(5);
        let client_row = Row::new()
            .push(client_ip_text_input)
            .push(client_port_text_input)
            .push(client_tcp_button)
            .push(client_udp_button)
            .padding(0)
            .spacing(2)
            .align_items(Align::Center)
            .max_height(1000000)
            .max_width(1000000);
        
        let client_column = Column::new()
            .push(client_text)
            .push(client_row)
            .padding(16)
            .spacing(12)
            .align_items(Align::Start)
            .width(Length::FillPortion(1))
            .max_height(1000000)
            .max_width(1000000); 

        let row = Row::new()
            .push(server_column)
            .push(client_column)
            .padding(16)
            .spacing(12)
            .align_items(Align::Start)
            .max_height(1000000)
            .max_width(1000000);

        Scrollable::new(&mut self.scrollable_state)
            .push(row)
            .into()
    }
}

fn generate_setting() -> Settings<()> {
    let mut setting = Settings::default();
    setting.window.size.0 = 800;
    setting.window.size.1 = 500;
    setting.window.resizable = true;
    setting.window.min_size = Option::Some((800, 500));

    setting
}
pub fn main() -> iced::Result {
    Rokit::run(generate_setting())
}

