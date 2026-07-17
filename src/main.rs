use iced::widget::{button, column, text};
use iced::{Center, Element, Task};

pub fn main() -> iced::Result {
    iced::application("Counter - Iced Demo", Counter::update, Counter::view).run()
}

/// Application state
#[derive(Default)]
struct Counter {
    value: i64,
}

/// All events the application can produce
#[derive(Debug, Clone)]
enum Message {
    Increment,
    Decrement,
}

impl Counter {
    /// Update state in response to a message.
    /// Returns a `Task` for any follow-up async work (none needed here).
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Increment => self.value += 1,
            Message::Decrement => self.value -= 1,
        }
        Task::none()
    }

    /// Build the widget tree from the current state.
    fn view(&self) -> Element<'_, Message> {
        column![
            button("Increment").on_press(Message::Increment),
            text(self.value).size(50),
            button("Decrement").on_press(Message::Decrement),
        ]
        .padding(20)
        .align_x(Center)
        .into()
    }
}
