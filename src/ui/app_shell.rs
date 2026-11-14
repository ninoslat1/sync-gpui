use gpui_kit::{Context, Entity, IntoElement, Render, Subscription, Window, *};

use crate::{libs::keyring::load_config, repository::location_repository, types::{DBConfigForm, PosConfigForm}, ui::configuration_db::DbConfigSaved};

enum Screen {
    Db(Entity<DBConfigForm>),
    Pos(Entity<PosConfigForm>),
}

pub struct AppShell {
    screen: Screen,
    _sub: Option<Subscription>
}

impl AppShell {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        if matches!(load_config(), Ok(Some(_))) {
            let pos = Self::create_pos(window, cx);

            Self {
                screen: Screen::Pos(pos),
                _sub: None,
            }
        } else {
            let db = cx.new(|cx| DBConfigForm::new(window, cx));

            let sub = cx.subscribe_in(
                &db,
                window,
                |this, _, _: &DbConfigSaved, window, cx| {
                    this.show_pos(window, cx);
                },
            );

            Self {
                screen: Screen::Db(db),
                _sub: Some(sub),
            }
        }
    }

    fn create_pos(
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Entity<PosConfigForm> {
        let locations = match load_config() {
            Ok(Some(cfg)) => {
                 location_repository::LocationRepository::connect(&cfg)
                .and_then(|repo| repo.load_location_list())
                .unwrap_or_default()
            }
            _ => Vec::new(),
        };
    
        cx.new(|cx| PosConfigForm::new(window, cx, locations))
    }

    fn show_pos(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let pos = Self::create_pos(window, cx);

        self.screen = Screen::Pos(pos);
        self._sub = None;

        cx.notify();
    }
}

impl Render for AppShell {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        match &self.screen {
            Screen::Db(v) => div().size_full().child(v.clone()),
            Screen::Pos(v) => div().size_full().child(v.clone()),
        }
    }
}