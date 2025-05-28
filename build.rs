use glob::glob;
use regex::Regex;
use std::env;
use std::fs;
use std::path::Path;
use std::collections::HashMap;

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("blog_posts.rs");
    
    let mut posts = Vec::new();
    let valid_filename = Regex::new(r"^[a-zA-Z0-9-]+-\d{4}-\d{2}-\d{2}\.md$").unwrap();
    
    for entry in glob("blogs/*.md").expect("Failed to read glob pattern") {
        let path = entry.unwrap();
        let filename = path.file_name().unwrap().to_str().unwrap();
        
        if !valid_filename.is_match(filename) {
            println!("cargo:warning=Skipping invalid filename: {}", filename);
            continue;
        }
        
        let content = fs::read_to_string(&path).unwrap();
        let mut metadata = HashMap::new();
        let mut lines = content.lines();
        let mut body = String::new();
        
        while let Some(line) = lines.next() {
            if line.starts_with("--") {
                let line = line.trim_start_matches("--").trim();
                if let Some((key, value)) = line.split_once(':') {
                    metadata.insert(key.trim().to_string(), value.trim().to_string());
                }
            } else {
                body = lines.collect::<Vec<_>>().join("\n");
                break;
            }
        }

        let image = metadata.get("image").cloned().unwrap_or_default();
        posts.push((
            filename.to_string(),
            metadata.get("title").cloned().unwrap_or_default(),
            metadata.get("date").cloned().unwrap_or_default(),
            image.clone(),
            body,
        ));
    }

    let entries_code: Vec<String> = posts
    .iter()
    .map(|(filename, title, date, image, content)| {
        format!(
            r#""{}" => BlogPostTransfer {{
                title: {:?},
                date: {:?},
                image: {:?},
                content: {:?},
                filename: {:?}
            }}"#,
            filename, title, date, image, content, filename
        )
    })
    .collect();


    fs::write(
        &dest_path,
        format!(
            r#"
            use phf::phf_map;
            
            #[derive(Debug, Clone)]
            pub struct BlogPostTransfer {{
                pub title: &'static str,
                pub date: &'static str,
                pub image: &'static str,
                pub content: &'static str,
                pub filename: &'static str,
            }}
    
            pub static BLOG_POSTS: phf::Map<&'static str, BlogPostTransfer> = phf_map! {{
                {}
            }};
            "#,
            entries_code.join(",\n")
        )
    ).unwrap();

    println!("cargo:rerun-if-changed=blogs/");
}