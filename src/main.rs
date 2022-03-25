mod tcp_client;
mod udp_client;
mod rokit_error;
mod common;

use chrono::Local;
use rokit_error::RokitError;
use tcp_client::TcpClient;
use iced::{button, executor, scrollable, text_input,
    Align, Application, Button, Command, Column, Clipboard, Element, Font, Settings, HorizontalAlignment, 
    Length, Row, Scrollable, Text, TextInput, VerticalAlignment};

const FZFONT: Font = Font::External {
    name: "方正字体",
    bytes: include_bytes!("source/FZFWZhuZGDLHJW.TTF"),
};

const CLIENT_TCP_BUTTON_TEXT_CONNECT : &str = "TCP连接";
const CLIENT_TCP_BUTTON_TEXT_DISCONNECT : &str = "TCP断开";
const CLIENT_UDP_BUTTON_TEXT_CONNECT : &str = "UDP连接";
const CLIENT_UDP_BUTTON_TEXT_DISCONNECT : &str = "UDP断开";

struct Rokit{

    client_ip_text_input_state: text_input::State,
    client_ip_text_input: String,
    client_port_text_input_state: text_input::State,
    client_port_text_input: String,
    client_tcp_button_text:String,
    client_tcp_button_state: button::State,

    client_udp_button_text:String,
    client_udp_button_state: button::State,

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

    tcp_client:Option<TcpClient>
}

#[derive(Debug, Clone)]
struct TcpClientResult {
    result:String,
    client:TcpClient
}

#[derive(Debug, Clone)]
enum RokitMessage {
    ClientIPTextInput(String),
    ClientPortTextInput(String),
    ClientTCPButton,
    ClientUDPButton,

    ClientBufferTextInput(String),
    ClientASCIIBufferTextInput(String),
    ClientSendButton,
    ClientASCIISendButton,

    ReadTcpClient(Result<TcpClientResult, RokitError>),
}

impl Rokit {
    async fn read_tcp_client(mut tcp_client: TcpClient) -> Result<TcpClientResult, RokitError>{
        match tcp_client.read() {
            Ok(s) => Ok(TcpClientResult{client:tcp_client, result:s}),
            Err(e) => Err(e)
        }
    }
}

impl Application for Rokit {
    type Executor = executor::Default;
    type Message = RokitMessage;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Rokit, Command<Self::Message>) {
        (
            Rokit{
                client_ip_text_input_state: text_input::State::new(),
                client_ip_text_input: String::from("127.0.0.1"),
                client_port_text_input_state: text_input::State::new(),
                client_port_text_input: String::from("8888"),
                client_tcp_button_text:String::from(CLIENT_TCP_BUTTON_TEXT_CONNECT),
                client_tcp_button_state: button::State::new(),

                client_udp_button_text:String::from(CLIENT_UDP_BUTTON_TEXT_CONNECT),
                client_udp_button_state: button::State::new(),

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

                tcp_client:None,

            }, 
            Command::none()
        )
    }

    fn title(&self) -> String {
        String::from("Rokit")
    }

    fn update(&mut self, message: Self::Message, _clipboard: &mut Clipboard) -> Command<Self::Message> {
        match message {

            RokitMessage::ClientIPTextInput(s) => {
                self.client_ip_text_input = s;
                Command::none()
            },
            RokitMessage::ClientPortTextInput(s) => {
                self.client_port_text_input = s;
                Command::none()
            },
            RokitMessage::ClientTCPButton => {
                match self.tcp_client {
                    Some(ref mut client) => {
                        match client.disconnect() {
                            Ok(_) => {},
                            Err(e) => {
                                self.client_output_text += generate_log(e.msg).as_str();
                            }
                        }
                        self.tcp_client = None;
                        self.client_tcp_button_text = String::from(CLIENT_TCP_BUTTON_TEXT_CONNECT);
                        Command::none()
                    },
                    None => {
                        let new_tcp_client = TcpClient::connect(self.client_ip_text_input.clone(), self.client_port_text_input.clone());
                        match new_tcp_client {
                            Ok(c) => {
                                let cl = c.clone();
                                self.client_output_text += generate_log(format!("TCP连接:{} {}", c.socket_addr.ip().to_string(), c.socket_addr.port())).as_str();
                                self.tcp_client = Some(c);
                                self.client_tcp_button_text = String::from(CLIENT_TCP_BUTTON_TEXT_DISCONNECT);
                                Command::perform(Rokit::read_tcp_client(cl), RokitMessage::ReadTcpClient)
                            },
                            Err(e) => {
                                self.client_output_text += generate_log(e.msg).as_str();
                                Command::none()
                            }
                        }
                    }
                }
            },
            RokitMessage::ClientUDPButton => {
                self.client_udp_button_text = String::from(CLIENT_UDP_BUTTON_TEXT_DISCONNECT);
                Command::none()
            },

            RokitMessage::ClientBufferTextInput(s) => {
                self.client_buffer_text_input = s;
                Command::none()
            },
            RokitMessage::ClientASCIIBufferTextInput(s) => {
                self.client_ascii_buffer_text_input = s;
                Command::none()
            },
            RokitMessage::ClientSendButton => {
                match self.tcp_client.as_mut() {
                    Some(client) => {
                        match client.send(self.client_buffer_text_input.clone()) {
                            Ok(x) => self.client_output_text += generate_log(format!("已发送{}字节:{}", x, self.client_buffer_text_input)).as_str(),
                            Err(e) => {
                                self.client_output_text += generate_log(e.msg).as_str();
                                match client.disconnect() {
                                    Ok(_) => {},
                                    Err(e) => {
                                        self.client_output_text += generate_log(e.msg).as_str();
                                    }
                                }
                                self.tcp_client = None;
                                self.client_tcp_button_text = String::from(CLIENT_TCP_BUTTON_TEXT_CONNECT);
                            }
                        }
                    },
                    None => {
                        self.client_output_text +=  generate_log("TCP未连接".to_string()).as_str();
                    }
                } 
                Command::none()
            },
            RokitMessage::ClientASCIISendButton => {
                println!("已发送{}", self.client_ascii_buffer_text_input);
                Command::none()
            },
            RokitMessage::ReadTcpClient(result) => {
                match result {
                    Ok(x) => {
                        self.client_output_text += generate_log(format!("收到:{}", x.result)).as_str();
                        match self.tcp_client{
                            Some(_) => {
                                Command::perform(Rokit::read_tcp_client(x.client), RokitMessage::ReadTcpClient)
                            },
                            None => {
                                Command::none()
                            }
                        }
                    }
                    Err(e) => {
                        self.client_output_text += generate_log(e.msg).as_str();

                        match self.tcp_client.as_mut() {
                            Some(client) => {
                                match client.disconnect() {
                                    Ok(_) => {},
                                    Err(e) => {
                                        self.client_output_text += generate_log(e.msg).as_str();
                                    }
                                }
                                self.tcp_client = None;
                                self.client_tcp_button_text = String::from(CLIENT_TCP_BUTTON_TEXT_CONNECT);
                            },
                            None => {}
                        }
                        Command::none()
                    }
                }
            }
        }
    }

    fn view(&mut self) -> Element<Self::Message> {
        
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

        Scrollable::new(&mut self.scrollable_state)
            .push(client_column)
            .into()
    }
}

fn generate_log(msg:String) -> String {

    let fmt = "%H:%M:%S";
    let date_str = Local::now().format(fmt).to_string();
    return date_str + " " + msg.as_str() + "\n";
}

fn generate_setting() -> Settings<()> {
    let mut setting = Settings::default();
    setting.window.size.0 = 400;
    setting.window.size.1 = 500;
    setting.window.resizable = true;
    setting.window.min_size = Option::Some((400, 500));

    setting
}
pub fn main() -> iced::Result {
    Rokit::run(generate_setting())
}

