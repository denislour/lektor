use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest = Path::new(&out_dir).join("posts_list.rs");

    let posts_dir = Path::new(match env::var("LEKTOR_ENV") {
        Ok(v) if v == "production" => "posts",
        _ => "posts-local",
    });

    let mut entries: Vec<String> = Vec::new();

    if let Ok(dir) = fs::read_dir(posts_dir) {
        for entry in dir.flatten() {
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) == Some("md") {
                if let Some(name) = path.file_stem().and_then(|n| n.to_str()) {
                    entries.push(name.to_string());
                }
            }
        }
    }

    entries.sort();

    let mut code = String::new();
    code.push_str(&format!(
        "pub const ALL_POSTS: [(&str, &str); {}] = [\n",
        entries.len()
    ));

    for id in &entries {
        let abs_path = fs::canonicalize(posts_dir.join(format!("{}.md", id)))
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_default();
        code.push_str(&format!("    ({:?}, include_str!({:?})),\n", id, abs_path));
    }

    code.push_str("];\n");
    fs::write(&dest, &code).unwrap();
}
