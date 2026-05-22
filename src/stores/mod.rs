mod app_store;
mod posts_store;
mod search_store;

pub use app_store::AppStore;
pub use posts_store::PostsStore;
pub use search_store::SearchStore;

#[derive(Clone, Copy)]
pub struct AppCtx {
    pub app: AppStore,
    pub search: SearchStore,
    pub posts: PostsStore,
}
