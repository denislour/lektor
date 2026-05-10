# Lektor

A **backend & systems blog** built with [Leptos](https://leptos.dev/) (Rust WASM),
featuring a collection of posts on Rust, Python, databases, DevOps, and system design.

## Tech Stack

| Layer | Technology |
|-------|-----------|
| **Frontend** | [Leptos 0.8](https://leptos.dev/) (CSR) + Tailwind CSS 4 |
| **Bundler** | [Trunk](https://trunkrs.dev/) |
| **Markdown** | [comrak](https://github.com/kivikakk/comrak) (build-time) |
| **Syntax Highlighting** | highlight.js 11 |
| **Deployment** | Cloudflare Workers (static assets) |
| **CI** | GitHub Actions |

## Features

- 📝 Blog posts from Markdown files with YAML frontmatter
- 🔍 Full-text search across posts
- 🌗 Dark/light theme (persisted to localStorage)
- 🌐 English / Vietnamese i18n
- 📄 Pagination (6 posts per page)
- ⌨️ Code syntax highlighting (highlight.js)
- ⬆️ Back-to-top button
- 📱 Responsive design with Tailwind CSS
- ♿ Accessible with proper ARIA labels

## Project Structure

```
src/
├── app.rs             # Root component with router
├── lib.rs             # WASM entry point
├── components/        # Reusable UI components
│   ├── header.rs
│   ├── hero.rs
│   ├── featured.rs
│   ├── post_card.rs
│   ├── pagination.rs
│   └── footer.rs
├── pages/             # Route pages
│   ├── home.rs
│   ├── post.rs        # Post detail page
│   └── about.rs
├── stores/            # Reactive stores
│   ├── app_store.rs   # Theme, language, UI state
│   ├── posts_store.rs # Post data & search
│   └── search_store.rs
├── utils/             # Utilities organized by concern
│   ├── storage.rs     # localStorage helpers
│   ├── session.rs     # sessionStorage helpers
│   ├── doc.rs         # Document attribute helpers
│   ├── hljs.rs        # Code highlighting (highlight.js)
│   ├── scroll.rs      # Scroll watching with cleanup
│   └── helpers.rs     # Tag mapping, scroll position
├── data.rs            # Post types & rendering
├── i18n.rs            # Internationalization
└── app.css            # Tailwind + custom theme

posts/                 # Blog posts (Markdown + frontmatter)
```

## Getting Started

### Prerequisites

- [Rust](https://rustup.rs/) (with `wasm32-unknown-unknown` target)
- [Trunk](https://trunkrs.dev/)

```bash
# Install WASM target
rustup target add wasm32-unknown-unknown

# Install Trunk
cargo install trunk
```

### Development

```bash
# Start dev server with hot reload
trunk serve

# Build for production
trunk build --release
```

### Deploy

```bash
# Build then deploy to Cloudflare Workers
npx wrangler pages deploy ./dist
```

## Blog Posts

Posts are stored as Markdown files in `posts/` with the format:

```yaml
---
title: "Post Title"
date: "2026-05-10"
tags: ["rust", "tokio"]
readTime: 8
excerpt: "A brief summary..."
thumbnailCode: "rust"
---

## Post content here...
```

They are compiled into a static module at build time via `build.rs`.

## License

MIT
