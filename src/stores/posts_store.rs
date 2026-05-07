use crate::data::Post;
use leptos::prelude::*;
use reactive_stores::Store;

#[derive(Store)]
struct PostsState {
    all: Vec<Post>,
    items: Vec<Post>,
}

#[derive(Clone, Copy)]
pub struct PostsStore {
    state: Store<PostsState>,
}

impl PostsStore {
    pub fn new(posts: Vec<Post>) -> Self {
        Self {
            state: Store::new(PostsState {
                all: posts.clone(),
                items: posts,
            }),
        }
    }

    pub fn items(&self) -> impl Get<Value = Vec<Post>> + Copy + use<> {
        self.state.items()
    }

    pub fn latest(&self) -> Option<Post> {
        self.state.all().get().into_iter().next()
    }

    pub fn filter(&self, query: &str) {
        match query.trim().is_empty() {
            true => {
                let all = self.state.all().get();
                self.state.items().set(all);
            }
            false => {
                let ql = query.to_lowercase();
                let all = self.state.all().get();
                let filtered: Vec<Post> = all
                    .into_iter()
                    .filter(|p| {
                        p.title.to_lowercase().contains(&ql)
                            || p.excerpt.to_lowercase().contains(&ql)
                            || p.tags.iter().any(|t| t.to_lowercase().contains(&ql))
                            || p.content.to_lowercase().contains(&ql)
                    })
                    .collect();
                self.state.items().set(filtered);
            }
        }
    }
}
