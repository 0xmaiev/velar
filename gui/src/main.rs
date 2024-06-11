use iced_core::window::Level;
use solana_client::client_error;
use theme::ThemeProvider;
use {
    app::{Config, RpcProvider, Velar},
    iced::{window::Settings as WindowSettings, Application, Settings as ApplicationSettings},
    iced_core::Size,
};

mod app;
mod commands;
mod constants;
mod theme;
mod update;
mod widgets;

//pub type Result<T, E = anyhow::Error> = std::result::Result<T, E>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Iced(#[from] iced::Error),
    #[error(transparent)]
    Client(#[from] client_error::ClientError),
}

#[tokio::main(flavor = "multi_thread")]
pub async fn main() -> Result<(), Error> {
    let rpc_provider = RpcProvider::default();
    let theme_provider = ThemeProvider::default();

    let flags = Config {
        rpc_provider,
        theme_provider,
    };

    match Velar::run(ApplicationSettings {
        id: Some("Velar".to_string()),
        window: WindowSettings {
            size: Size {
                width: 1280.0,
                height: 720.0,
            },
            min_size: Some(Size {
                width: 1280.0,
                height: 720.0,
            }),
            position: iced::window::Position::Centered,
            visible: true,
            resizable: false,
            transparent: true,
            decorations: true,
            level: Level::AlwaysOnTop,
            ..WindowSettings::default()
        },
        antialiasing: true,
        flags,
        ..ApplicationSettings::default()
    }) {
        Ok(()) => Ok(()),
        Err(e) => Err(Error::Iced(e)),
    }
}
