use gpui_kit::{Context, Entity, IntoElement, Render, Subscription, Window, *};

use crate::{libs::keyring::load_config, types::{DBConfigForm, PosConfigForm}, ui::configuration_db::DbConfigSaved};

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
            let pos = cx.new(|cx| PosConfigForm::new(window, cx));
            Self {screen: Screen::Pos(pos), _sub: None}
        } else {
            let db = cx.new(|cx| DBConfigForm::new(window, cx));
            let sub = cx.subscribe_in(&db, window, |this, _, _: &DbConfigSaved, window, cx| {
                this.show_pos(window, cx);
            });
            Self { screen: Screen::Db(db), _sub: Some(sub) }
        }
    }

    fn show_pos(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let pos = cx.new(|cx| PosConfigForm::new(window, cx));
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