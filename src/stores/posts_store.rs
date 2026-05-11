use crate::data::Post;
use leptos::prelude::*;
use reactive_stores::Store;

#[derive(Store)]
struct PostsState {
    all: Vec<Post>,
}

#[derive(Clone, Copy)]
pub struct PostsStore {
    state: Store<PostsState>,
}

impl PostsStore {
    pub fn new(posts: Vec<Post>) -> Self {
        Self {
            state: Store::new(PostsState { all: posts }),
        }
    }

    pub fn items(
        &self,
        search_query: impl Get<Value = String> + Copy + Send + Sync + 'static,
    ) -> Signal<Vec<Post>> {
        let s = self.state;
        Signal::derive(move || {
            let q = search_query.get().trim().to_lowercase();
            s.all()
                .get()
                .into_iter()
                .filter(|p| {
                    q.is_empty()
                        || p.title.to_lowercase().contains(&q)
                        || p.excerpt.to_lowercase().contains(&q)
                        || p.tags.iter().any(|t| t.to_lowercase().contains(&q))
                        || p.content.to_lowercase().contains(&q)
                })
                .collect()
        })
    }

    pub fn latest(&self) -> Option<Post> {
        self.state.all().get().into_iter().next()
    }
}
