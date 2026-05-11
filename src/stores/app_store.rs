use crate::i18n::Lang;
use crate::utils::{Session, Storage};
use leptos::prelude::*;
use reactive_stores::Store;

#[derive(Store)]
struct AppState {
    theme: String,
    lang: Lang,
    page: usize,
    show_back_to_top: bool,
    menu_open: bool,
}

#[derive(Clone, Copy)]
pub struct AppStore {
    state: Store<AppState>,
}

impl AppStore {
    pub fn new(theme: &str, lang: Lang) -> Self {
        let page = Session::get_item("lektor_page")
            .and_then(|p| p.parse::<usize>().ok())
            .unwrap_or(1);
        Self {
            state: Store::new(AppState {
                theme: theme.to_string(),
                lang,
                page,
                show_back_to_top: false,
                menu_open: false,
            }),
        }
    }

    // Getters

    pub fn theme(&self) -> impl Get<Value = String> + Copy + use<> {
        self.state.theme()
    }

    pub fn lang(&self) -> impl Get<Value = Lang> + Copy + use<> {
        self.state.lang()
    }

    pub fn show_back_to_top(&self) -> impl Get<Value = bool> + Copy + use<> {
        self.state.show_back_to_top()
    }

    pub fn menu_open(&self) -> impl Get<Value = bool> + Copy + use<> {
        self.state.menu_open()
    }

    pub fn page(&self) -> impl Get<Value = usize> + Copy + use<> {
        self.state.page()
    }

    // Actions (custom logic, localStorage)

    pub fn set_page(&self, n: usize) {
        self.state.page().set(n);
    }

    pub fn toggle_theme(&self) {
        let theme = match self.state.theme().get() == "light" {
            true => "dark",
            false => "light",
        };
        self.state.theme().set(theme.to_string());
        Storage::set_item("theme", &theme);
    }

    pub fn toggle_lang(&self) {
        let (k, v) = match self.state.lang().get() == Lang::En {
            true => (Lang::Vi, "vi"),
            false => (Lang::En, "en"),
        };
        self.state.lang().set(k);
        Storage::set_item("lang", v);
    }

    pub fn toggle_menu(&self) {
        let open = self.state.menu_open().get();
        self.state.menu_open().set(!open);
    }

    pub fn set_back_to_top(&self, visible: bool) {
        self.state.show_back_to_top().set(visible);
    }
}
