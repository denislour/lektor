use leptos::prelude::*;
use reactive_stores::Store;

#[derive(Store)]
struct SearchState {
    query: String,
}

#[derive(Clone, Copy)]
pub struct SearchStore {
    state: Store<SearchState>,
}

impl SearchStore {
    pub fn new() -> Self {
        Self {
            state: Store::new(SearchState {
                query: String::new(),
            }),
        }
    }

    // Getters

    pub fn query(&self) -> impl Get<Value = String> + Copy + use<> {
        self.state.query()
    }

    pub fn is_searching(&self) -> Signal<bool> {
        let store = self.state;
        Signal::derive(move || !store.query().get().trim().is_empty())
    }

    // Actions

    pub fn set_query(&self, q: &str) {
        self.state.query().set(q.to_string());
    }
}
