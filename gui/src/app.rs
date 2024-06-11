use iced::{
    event::{self, Event},
    executor,
    widget::vertical_space,
    Application, Command, Element,
};
use iced_core::{alignment::Vertical, Alignment, Length};
use iced_style::Theme;
use iced_widget::{column, container, horizontal_space, row};
use solana_client::nonblocking::{pubsub_client::PubsubClient, rpc_client::RpcClient};
use solana_sdk::commitment_config::CommitmentConfig;
use std::{
    sync::Arc,
    time::{Duration, SystemTime},
};

use crate::{
    constants::{JSON_RPC_URL, PUBSUB_RPC_URL},
    theme::ThemeProvider,
    update::handle_update,
    widgets,
};

#[derive(Debug, Clone)]
pub enum Message {
    Default,
    AccountCreated(String),
    EventOccurred(Event),
}

impl Default for Message {
    fn default() -> Self {
        Self::Default
    }
}

#[derive(Debug, Default, Clone)]
pub struct Config {
    pub rpc_provider: RpcProvider,
    pub theme_provider: ThemeProvider,
}

#[derive(Clone)]
pub struct RpcProvider {
    pub rpc_url: String,
    pub pubsub_url: String,
    pub rpc_client: Arc<RpcClient>,
    pub pubsub_client: Arc<PubsubClient>,
}

impl std::fmt::Debug for RpcProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RpcProvider")
            .field("rpc_url", &format!("{}", self.rpc_url))
            .field("rpc_client", &format!("{}", self.rpc_client.url()))
            .field("pubsub_url", &format!("{}", self.pubsub_url))
            .finish()
    }
}

impl Default for RpcProvider {
    fn default() -> Self {
        let pubsub_client =
            Arc::new(futures::executor::block_on(PubsubClient::new(PUBSUB_RPC_URL)).unwrap());
        Self {
            rpc_url: JSON_RPC_URL.to_string(),
            pubsub_url: PUBSUB_RPC_URL.to_string(),
            rpc_client: Arc::new(RpcClient::new_with_commitment(
                JSON_RPC_URL.to_string(),
                CommitmentConfig::confirmed(),
            )),
            pubsub_client,
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct AccountCreationState {
    pub alias: String,
    pub class: String,
}

#[derive(Debug, Clone)]
pub enum State {
    Loading,
    CreatingAccount(AccountCreationState),
}

impl Default for State {
    fn default() -> Self {
        Self::Loading
    }
}

#[derive(Debug, Default, Clone)]
pub struct Velar {
    rpc_provider: RpcProvider,
    theme_provider: ThemeProvider,
    pub state: State,
}

impl Velar {
    pub fn apply_config(&mut self, config: Config) {
        self.rpc_provider = config.rpc_provider;
        self.theme_provider = config.theme_provider;
    }
}

impl Application for Velar {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = Config;

    fn new(flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        let mut app = Velar::default();

        app.apply_config(flags);

        (app, Command::none())
    }

    fn title(&self) -> String {
        String::from("Velar")
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        let init = SystemTime::now();
        let cmd = match handle_update(self, message) {
            Ok(cmd) => cmd,
            Err(e) => {
                log::error!("Error during update handling: {:?}", e);
                return Command::none();
            } // Command::perform(async { e }, Message::Error),
        };
        let end = SystemTime::now();
        let elapsed = end.duration_since(init).unwrap();
        log::info!("Update Elapsed {}", elapsed.as_micros());
        cmd
    }

    fn view(&self) -> iced::Element<'_, Self::Message, Self::Theme, iced::Renderer> {
        let init = SystemTime::now();

        const DEFAULT_MAIN_COLUMN_PADDING: u16 = 10;
        const DEFAULT_MAIN_COLUMN_SPACING: u16 = 10;

        const DEFAULT_MAIN_ROW_PADDING: u16 = 10;
        const DEFAULT_MAIN_ROW_SPACING: u16 = 10;

        let content = match &self.state {
            State::Loading => {
                let spinner: Element<Message, Theme, iced::Renderer> = widgets::Circular::new()
                    .easing(&widgets::easing::STANDARD)
                    .cycle_duration(Duration::from_secs(3))
                    .into();
                let content = row![
                    horizontal_space(),
                    column![spinner,]
                        .padding(DEFAULT_MAIN_COLUMN_PADDING)
                        .spacing(DEFAULT_MAIN_COLUMN_PADDING)
                        .align_items(Alignment::Center)
                        .width(Length::Fill)
                        .height(Length::Fill),
                    horizontal_space()
                ]
                .align_items(Alignment::Center)
                .spacing(DEFAULT_MAIN_ROW_SPACING)
                .padding(DEFAULT_MAIN_ROW_PADDING)
                .width(Length::Fill);

                column![vertical_space(), content, vertical_space()]
                    .align_items(Alignment::Center)
                    .spacing(DEFAULT_MAIN_COLUMN_SPACING)
                    .padding(DEFAULT_MAIN_COLUMN_PADDING)
                    .width(Length::Fill)
            }
            State::CreatingAccount(state) => todo!(),
        };

        let end = SystemTime::now();
        let elapsed = end.duration_since(init).unwrap();
        log::info!("View Elapsed {}", elapsed.as_micros());

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .align_y(Vertical::Top)
            .into()
    }

    fn subscription(&self) -> iced_futures::Subscription<Self::Message> {
        event::listen().map(Message::EventOccurred)
    }

    fn theme(&self) -> Self::Theme {
        self.theme_provider.theme()
    }
}
