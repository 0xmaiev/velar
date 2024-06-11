use crate::{
    app::{Message, Velar},
    Error,
};
use iced::Command;
use iced_core::{keyboard::key::Named, Event};
//use utils::Direction;

#[derive(Debug, Clone)]
pub enum Actions {
    //MoveTowards(Direction, u64),
}

pub fn handle_update(app: &mut Velar, message: Message) -> Result<Command<Message>, Error> {
    use Message::*;
    match message {
        Default => (),
        EventOccurred(event) => match event {
            Event::Keyboard(keyboard) => {
                if let iced_core::keyboard::Event::KeyPressed { key, .. } = keyboard {
                    if let iced_core::keyboard::Key::Named(named_key) = key {
                        // use Named::*;
                        // let direction = match named_key {
                        //     ArrowUp => Direction::North,
                        //     ArrowDown => Direction::South,
                        //     ArrowLeft => Direction::West,
                        //     ArrowRight => Direction::East,
                        //     _ => {
                        //         return Ok(Command::none());
                        //     }
                        // };

                        return Ok(Command::none());
                    } else {
                        return Ok(Command::none());
                    }
                }
            }
            _ => {
                return Ok(Command::none());
            }
        },
        AccountCreated(account) => match app.state {
            crate::app::State::Loading => todo!(),
            crate::app::State::CreatingAccount(_) => todo!(),
        },
    };
    Ok(Command::none())
}
