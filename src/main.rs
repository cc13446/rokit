mod server;
mod client;
use iced::{button, executor, scrollable, text_input,
    Align, Application, Button, Checkbox, Command, Column, Clipboard, Element, Font, Settings, HorizontalAlignment, 
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


    addrs: Vec<Addr>,
    addrs_scrollable_state:scrollable::State,

    server_all_button_text:String,
    server_all_button_state: button::State,
    server_disconnect_button_text:String,
    server_disconnect_button_state: button::State,

    server_buffer_text_input_state: text_input::State,
    server_buffer_text_input: String,
    server_send_button_text:String,
    server_send_button_state: button::State,

    server_ascii_buffer_text_input_state: text_input::State,
    server_ascii_buffer_text_input: String,
    server_ascii_send_button_text:String,
    server_ascii_send_button_state: button::State,

    server_output_text:String,
    server_output_scrollable_state:scrollable::State,


    client_buffer_text_input_state: text_input::State,
    client_buffer_text_input: String,
    client_send_button_text:String,
    client_send_button_state: button::State,

    client_ascii_buffer_text_input_state: text_input::State,
    client_ascii_buffer_text_input: String,
    client_ascii_send_button_text:String,
    client_ascii_send_button_state: button::State,

    client_output_text:String,
    client_output_scrollable_state:scrollable::State,

    scrollable_state:scrollable::State,
}

struct Addr {
    ip: String,
    port: String,
    describe: String, 
    check_state: AddrState
}

#[derive(PartialEq)]
enum AddrState {
    Checked,
    Unchecked
}
#[derive(Debug, Clone)]
enum AddrMessage {
    Click(bool)
}

impl Addr {
    fn new (ip:String, port:String) -> Self {
        Addr{
            describe:String::from(ip.clone() + " " + (port.clone().as_str())),
            ip,
            port,
            check_state:AddrState::Unchecked
        }
    }
    fn update(&mut self, message: AddrMessage) {
        match message {
            AddrMessage::Click(flag) => {
                if flag {
                    self.check_state = AddrState::Checked;
                } else {
                    self.check_state = AddrState::Unchecked;
                }
            }
        }
    }
    fn view(&mut self) -> Element<AddrMessage> {

        let checkbox = Checkbox::new(self.check_state == AddrState::Checked,&self.describe,AddrMessage::Click)
            .width(Length::Fill)
            .text_size(18)
            .size(18);
    
        Row::new().spacing(20)
            .align_items(Align::Center)
            .push(checkbox)
            .into()
    }

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

    AddrMessage(usize, AddrMessage),
    ServerAllButton,
    ServerDisconnectButton,

    ServerBufferTextInput(String),
    ServerASCIIBufferTextInput(String),
    ServerSendButton,
    ServerASCIISendButton,

    ClientBufferTextInput(String),
    ClientASCIIBufferTextInput(String),
    ClientSendButton,
    ClientASCIISendButton,
    
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

                addrs:vec![Addr::new(String::from("127.0.0.1"), String::from("8888")), 
                    Addr::new(String::from("127.0.0.1"), String::from("8878")),
                    Addr::new(String::from("127.0.0.1"), String::from("8868")),
                    Addr::new(String::from("127.0.0.1"), String::from("8858")),],
               
                addrs_scrollable_state: scrollable::State::new(),

                server_all_button_text:String::from("选择全部"),
                server_all_button_state: button::State::new(),
                server_disconnect_button_text:String::from("断开链接"),
                server_disconnect_button_state: button::State::new(),

                server_buffer_text_input_state: text_input::State::new(),
                server_buffer_text_input: String::from(""),
                server_send_button_text:String::from("发送(UTF-8)"),
                server_send_button_state: button::State::new(),
            
                server_ascii_buffer_text_input_state: text_input::State::new(),
                server_ascii_buffer_text_input: String::from(""),
                server_ascii_send_button_text:String::from("发送(ASCII)"),
                server_ascii_send_button_state: button::State::new(),
            
                server_output_text:String::from(""),
                server_output_scrollable_state:scrollable::State::new(),

                client_buffer_text_input_state: text_input::State::new(),
                client_buffer_text_input: String::from(""),
                client_send_button_text:String::from("发送(UTF-8)"),
                client_send_button_state: button::State::new(),
            
                client_ascii_buffer_text_input_state: text_input::State::new(),
                client_ascii_buffer_text_input: String::from(""),
                client_ascii_send_button_text:String::from("发送(ASCII)"),
                client_ascii_send_button_state: button::State::new(),
            
                client_output_text:String::from(""),
                client_output_scrollable_state:scrollable::State::new(),

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
            RokitMessage::AddrMessage(i, addr_message) => {
                match self.addrs.get_mut(i) {
                    Some(addr) => {
                        addr.update(addr_message)
                    },
                    None => {}
                }
            },
            RokitMessage::ServerAllButton => {
                for addr in self.addrs.iter_mut() {
                    addr.check_state = AddrState::Checked
                }
            },
            RokitMessage::ServerDisconnectButton => {
                self.addrs.retain(|addr| addr.check_state==AddrState::Unchecked);
            },
            RokitMessage::ServerBufferTextInput(s) => {
                self.server_buffer_text_input = s;
            },
            RokitMessage::ServerASCIIBufferTextInput(s) => {
                self.server_ascii_buffer_text_input = s;
            },
            RokitMessage::ServerSendButton => {
                println!("已发送{}", self.server_buffer_text_input);
                self.server_output_text += ("\n".to_string() + self.server_buffer_text_input.clone().as_str()).as_str();
            },
            RokitMessage::ServerASCIISendButton => {
                println!("已发送{}", self.server_ascii_buffer_text_input)
            },
            RokitMessage::ClientBufferTextInput(s) => {
                self.client_buffer_text_input = s;
            },
            RokitMessage::ClientASCIIBufferTextInput(s) => {
                self.client_ascii_buffer_text_input = s;
            },
            RokitMessage::ClientSendButton => {
                println!("已发送{}", self.client_buffer_text_input);
                self.client_output_text += ("\n".to_string() + self.client_buffer_text_input.clone().as_str()).as_str();
            },
            RokitMessage::ClientASCIISendButton => {
                println!("已发送{}", self.client_ascii_buffer_text_input)
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
        
        let addrs: Element<_> = self.addrs
            .iter_mut()
            .enumerate()
            .fold(Column::new().spacing(2), 
            |column, (i, addr)| {
                column.push(addr.view().map(move |message| {
                    RokitMessage::AddrMessage(i, message)
                }))
            })
            .into();
        let addr_scrollable = Scrollable::new(&mut self.addrs_scrollable_state)
            .push(addrs)
            .max_height(58);
        
        let server_addrs_all_button_text = Text::new(&self.server_all_button_text)
            .font(FZFONT)
            .size(18)
            .vertical_alignment(VerticalAlignment::Center)
            .horizontal_alignment(HorizontalAlignment::Center);
        let server_addrs_all_button = Button::new(&mut self.server_all_button_state, server_addrs_all_button_text)
            .on_press(RokitMessage::ServerAllButton)
            .width(Length::FillPortion(1))
            .padding(5);
        let server_addrs_disconnect_button_text = Text::new(&self.server_disconnect_button_text)
            .font(FZFONT)
            .size(18)
            .vertical_alignment(VerticalAlignment::Center)
            .horizontal_alignment(HorizontalAlignment::Center);
        let server_addrs_disconnect_button = Button::new(&mut self.server_disconnect_button_state, server_addrs_disconnect_button_text)
            .on_press(RokitMessage::ServerDisconnectButton)
            .width(Length::FillPortion(1))
            .padding(5);

        let addrs_button_column = Column::new()
            .push(server_addrs_all_button)
            .push(server_addrs_disconnect_button)
            .max_width(80)
            .spacing(2);
        let addrs_row = Row::new()
            .push(addrs_button_column)
            .push(addr_scrollable)
            .spacing(2);
        
        let server_buffer_text_input = TextInput::new(&mut self.server_buffer_text_input_state, "msg",&self.server_buffer_text_input,RokitMessage::ServerBufferTextInput)
            .font(FZFONT)
            .width(Length::FillPortion(3)) 
            .padding(5);
        let server_send_button_text = Text::new(&self.server_send_button_text)
            .font(FZFONT)
            .size(16)
            .vertical_alignment(VerticalAlignment::Center)
            .horizontal_alignment(HorizontalAlignment::Center);
        let server_send_button = Button::new(&mut self.server_send_button_state, server_send_button_text)
            .on_press(RokitMessage::ServerSendButton)
            .width(Length::FillPortion(1))
            .padding(5);
        
        let buffer_row = Row::new()
            .push(server_buffer_text_input)
            .push(server_send_button)
            .align_items(Align::Center)
            .spacing(2);

        
        let server_ascii_buffer_text_input = TextInput::new(&mut self.server_ascii_buffer_text_input_state, "msg",&self.server_ascii_buffer_text_input,RokitMessage::ServerASCIIBufferTextInput)
            .font(FZFONT)
            .width(Length::FillPortion(3)) 
            .padding(5);
        let server_ascii_send_button_text = Text::new(&self.server_ascii_send_button_text)
            .font(FZFONT)
            .size(16)
            .vertical_alignment(VerticalAlignment::Center)
            .horizontal_alignment(HorizontalAlignment::Center);
        let server_ascii_send_button = Button::new(&mut self.server_ascii_send_button_state, server_ascii_send_button_text)
            .on_press(RokitMessage::ServerASCIISendButton)
            .width(Length::FillPortion(1))
            .padding(5);
        
        let ascii_buffer_row = Row::new()
            .push(server_ascii_buffer_text_input)
            .push(server_ascii_send_button)
            .align_items(Align::Center)
            .spacing(2);
        
        let server_output_text = Text::new(&self.server_output_text)
            .font(FZFONT)
            .size(17)
            .width(Length::Fill)
            .vertical_alignment(VerticalAlignment::Top)
            .horizontal_alignment(HorizontalAlignment::Left);

        let server_output_scrollable = Scrollable::new(&mut self.server_output_scrollable_state)
            .push(server_output_text)
            .max_height(160);
        
        let server_column = Column::new()
            .push(server_text)
            .push(server_tcp_row)
            .push(server_udp_row)
            .push(addrs_row)
            .push(buffer_row)
            .push(ascii_buffer_row)
            .push(server_output_scrollable)
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

        let client_buffer_text_input = TextInput::new(&mut self.client_buffer_text_input_state, "msg",&self.client_buffer_text_input,RokitMessage::ClientBufferTextInput)
            .font(FZFONT)
            .width(Length::FillPortion(3)) 
            .padding(5);
        let client_send_button_text = Text::new(&self.client_send_button_text)
            .font(FZFONT)
            .size(16)
            .vertical_alignment(VerticalAlignment::Center)
            .horizontal_alignment(HorizontalAlignment::Center);
        let client_send_button = Button::new(&mut self.client_send_button_state, client_send_button_text)
            .on_press(RokitMessage::ClientSendButton)
            .width(Length::FillPortion(1))
            .padding(5);
        
        let client_buffer_row = Row::new()
            .push(client_buffer_text_input)
            .push(client_send_button)
            .align_items(Align::Center)
            .spacing(2);

        
        let client_ascii_buffer_text_input = TextInput::new(&mut self.client_ascii_buffer_text_input_state, "msg",&self.client_ascii_buffer_text_input,RokitMessage::ClientASCIIBufferTextInput)
            .font(FZFONT)
            .width(Length::FillPortion(3)) 
            .padding(5);
        let client_ascii_send_button_text = Text::new(&self.client_ascii_send_button_text)
            .font(FZFONT)
            .size(16)
            .vertical_alignment(VerticalAlignment::Center)
            .horizontal_alignment(HorizontalAlignment::Center);
        let client_ascii_send_button = Button::new(&mut self.client_ascii_send_button_state, client_ascii_send_button_text)
            .on_press(RokitMessage::ClientASCIISendButton)
            .width(Length::FillPortion(1))
            .padding(5);
        
        let client_ascii_buffer_row = Row::new()
            .push(client_ascii_buffer_text_input)
            .push(client_ascii_send_button)
            .align_items(Align::Center)
            .spacing(2);
        
        let client_output_text = Text::new(&self.client_output_text)
            .font(FZFONT)
            .size(17)
            .width(Length::Fill)
            .vertical_alignment(VerticalAlignment::Top)
            .horizontal_alignment(HorizontalAlignment::Left);

        let client_output_scrollable = Scrollable::new(&mut self.client_output_scrollable_state)
            .push(client_output_text)
            .max_height(275);
        
        let client_column = Column::new()
            .push(client_text)
            .push(client_row)
            .push(client_buffer_row)
            .push(client_ascii_buffer_row)
            .push(client_output_scrollable)
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

