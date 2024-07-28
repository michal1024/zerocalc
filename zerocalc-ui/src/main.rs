use engine::{Calculator, Parser};
use iced::widget::{container, row, text, text_editor, Rule};
use iced::{self, Application, Command, Element, Length, Settings, Subscription};
use iced::time::Duration;

#[derive(Debug, Clone)]
enum Message {
    Edit(text_editor::Action),
    Evaluate,
}

struct Editor {
    content: text_editor::Content,
    result: String,
}

impl Editor {
    fn update_result(&mut self) {
        let mut calc = Calculator::new();
        self.result = String::new();
        for line in self.content.lines() {
            let mut parser = Parser::new(&line);
            match parser.parse() {
                Ok(true) => {
                    let num = calc.eval(&parser.program);
                    self.result.push_str(&(num.to_string() + "\n"))
                }
                Ok(false) => self.result.push('\n'),
                Err(err) => {
                    eprintln!("{:?}", err);
                    self.result.push_str("Error");
                }
            }
        };
    }
}

impl Application for Editor {
    type Message = Message;
    type Executor = iced::executor::Default;
    type Theme = iced::Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        let app = Editor {
            content: text_editor::Content::new(),
            result: String::new(),
        };
        (app, Command::none())
    }

    fn view(&self) -> Element<Self::Message> {
        let input = text_editor(&self.content)
            .height(Length::Fill)
            .padding(0)
            .on_action(Message::Edit);

        let output = text(self.result.clone()).size(16);

        row![
            container(input).padding(12).width(Length::FillPortion(3)),
            Rule::vertical(2),
            container(output).padding(12).width(Length::FillPortion(1))
        ]
        .into()
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Edit(action) => self.content.perform(action),
            Message::Evaluate => self.update_result(),
        };
        Command::none()
    }

    fn title(&self) -> String {
        String::from("Zerocalc")
    }

    fn theme(&self) -> iced::Theme {
        iced::Theme::Dark
    }

    fn subscription(&self) -> Subscription<Message> {
        iced::time::every(Duration::from_millis(250)).map(|_| { Message::Evaluate })
    }
}

fn main() -> iced::Result {
    Editor::run(Settings::default())
}

