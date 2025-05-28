use leptos::html::Pre;
use leptos::prelude::*;
use leptos_router::hooks::use_location;
use thiserror::Error;
use leptos_meta::{Meta, Title};

#[cfg(not(feature = "ssr"))]
use crate::hljs::highlight_all;


include!(concat!(env!("OUT_DIR"), "/blog_posts.rs"));

fn parse_md_content(content: &str) -> impl IntoView {
    use lazy_static::lazy_static;
    use regex::Regex;

    lazy_static! {
        static ref IMAGE_REGEX: Regex =
            Regex::new(r"^[^\s]+\.(png|jpg|jpeg|webp)$").unwrap();
        static ref LINK_REGEX: Regex =
            Regex::new(r"\[([^\]]+)\]\(([^)]+)\)").unwrap();
        // New regex for underscore-marked text
        static ref UNDERSCORE_REGEX: Regex =
            Regex::new(r"_(.*?)_").unwrap();
    }

    let lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();

    let mut output = Vec::new();
    let mut in_code_block = false;
    let mut code_block_buffer = Vec::new();
    let mut code_lang       = String::new();

    let el = NodeRef::<Pre>::new();
    Effect::new(move |_| {
        #[cfg(not(feature = "ssr"))]
        {
           if let Some(_) = el.get() {
               highlight_all();
           }
        }
    });

    for line in lines {

        if line.trim_start().starts_with("```") {
            if in_code_block { // Ending a code block
                let code = code_block_buffer.join("\n");
                let lang = code_lang.clone();
                output.push(
                    view! {
                        <pre
                            node_ref=el
                            class=String::from("code-block language-") + &lang
                            style="background: #1a1b26; padding: 1rem; border-radius: 4px; overflow: auto;"
                        >
                            <code>{code}</code>
                        </pre>
                    }.into_any()
                );
                code_block_buffer.clear();
                code_lang.clear();
            } else {
                // Starting a code block with optional language
                let lang = line.trim_start().trim_start_matches("```").trim();
                code_lang = lang.to_string();
            }
            in_code_block = !in_code_block;
            continue;
        }

        if in_code_block {
            code_block_buffer.push(line);
        } else {
            let l = line.trim();
            let element = match l {
                l if l.starts_with("### ") => {
                    view! { <h3>{l.strip_prefix("### ").unwrap_or(l).to_string()}</h3> }
                    .into_any()
                }
                l if l.starts_with("## ") => {
                    view! { <h2>{l.strip_prefix("## ").unwrap_or(l).to_string()}</h2> }
                    .into_any()
                }
                l if l.starts_with("# ") => {
                    view! { <h1>{l.strip_prefix("# ").unwrap_or(l).to_string()}</h1> }
                    .into_any()
                }
                l if l.starts_with("!image[") && l.ends_with(']') => {
                    let src = format!("{}", &l[7..l.len() - 1]);
                    view! { <img src=src class="content-image" /> }
                    .into_any()
                }
                l if l.starts_with("!youtube[") && l.ends_with(']') => {
                    // (YouTube branch code from your previous implementation)
                    let video_spec = l
                        .strip_prefix("!youtube[")
                        .and_then(|s| s.strip_suffix("]"))
                        .unwrap_or(l);

                    let video_id = if video_spec.contains("youtube.com") {
                        if let Some(idx) = video_spec.find("v=") {
                            let id = &video_spec[idx + 2..];
                            if let Some(end_idx) = id.find('&') {
                                id[..end_idx].to_string()
                            } else {
                                id.to_string()
                            }
                        } else {
                            video_spec.to_string()
                        }
                    } else if video_spec.contains("youtu.be/") {
                        if let Some(idx) = video_spec.rfind('/') {
                            video_spec[idx + 1..].to_string()
                        } else {
                            video_spec.to_string()
                        }
                    } else {
                        video_spec.to_string()
                    };

                    view! {
                        <iframe
                            class="iframe-md-video"
                            src=format!("https://www.youtube.com/embed/{}", video_id)
                            allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture"
                            allowfullscreen=true
                        ></iframe>
                    }
                    .into_any()
                }
                l if IMAGE_REGEX.is_match(l) => {
                    let src = format!("/{}", l);
                    view! { <img src=src class="content-image" /> }
                    .into_any()
                }
                l if l.starts_with('>') => view! {
                    <div class="quote-container">
                        <div class="quote-left-divider"></div>
                        <p class="quote">{l.strip_prefix('>').unwrap_or(l).trim().to_string()}</p>
                    </div>
                }
                .into_any(),
                // Updated default branch that also processes underscore-marked text
                l => {
                    // Helper closure to process plain text segments for underscore markup
                    let process_underscores = |text: &str| -> Vec<_> {
                        let mut parts = Vec::new();
                        let mut last_index = 0;
                        for cap in UNDERSCORE_REGEX.captures_iter(text) {
                            let m = cap.get(0).unwrap();
                            let start = m.start();
                            let end = m.end();

                            // Add text before the underscore segment if any
                            if start > last_index {
                                parts.push(
                                    view! { <>{text[last_index..start].to_string()}</> }
                                        .into_any(),
                                );
                            }
                            // The inner text between underscores
                            let inner_text = cap.get(1).unwrap().as_str().to_string();
                            parts.push(
                                view! { <span class="marked-md-text">{inner_text}</span> }
                                    .into_any(),
                            );
                            last_index = end;
                        }
                        // Append any remaining text after the last underscore match
                        if last_index < text.len() {
                            parts.push(
                                view! { <>{text[last_index..].to_string()}</> }
                                    .into_any(),
                            );
                        }
                        parts
                    };

                    let mut children = Vec::new();
                    let mut last_end = 0;

                    for cap in LINK_REGEX.captures_iter(l) {
                        let m = cap.get(0).unwrap();
                        let start = m.start();
                        let end = m.end();

                        if start > last_end {
                            // Process the substring for underscore markers before a link
                            children.extend(process_underscores(&l[last_end..start]));
                        }

                        let text = cap.get(1).unwrap().as_str().to_string();
                        let url = cap.get(2).unwrap().as_str().to_string();
                        children.push(
                            view! { <a href=url>{text}</a> }
                                .into_any(),
                        );

                        last_end = end;
                    }

                    if last_end < l.len() {
                        // Process any remaining text for underscores after the last link
                        children.extend(process_underscores(&l[last_end..]));
                    }

                    view! { <p>{children}</p> }.into_any()
                }
            };

            output.push(element);
        }
    }

    if in_code_block && !code_block_buffer.is_empty() {
        let code_content = code_block_buffer.join("\n");
        output.push(
            view! {
                <pre
                    class="code-block"
                    style="background: #24283b; padding: 1rem; border-radius: 4px; overflow: auto;"
                >
                    <code>{code_content}</code>
                </pre>
            }
            .into_any(),
        );
    }

    output
}


fn get_blog_post(filename: String) -> Result<BlogPost, ServerFnError> {
    use regex::Regex;

    let valid_format = Regex::new(r"^[a-zA-Z0-9-]+-\d{4}-\d{2}-\d{2}\.md$")
        .map_err(|_| BlogError::Io("Invalid regex pattern".to_string()))?;

    if !valid_format.is_match(&filename) {
        return Err(BlogError::Io(
            "Invalid filename format. Expected: {name}-{year}-{month}-{day}.md".to_string(),
        )
        .into());
    }

    let blog_post = BLOG_POSTS.get(&filename.as_str()).unwrap();

    Ok(BlogPost {
        title: blog_post.title.to_string(),
        date: blog_post.date.to_string(),
        image: blog_post.image.to_string(),
        content: blog_post.content.to_string(),
        filename: blog_post.filename.to_string(),
    })
}

#[component]
pub fn BlogView() -> impl IntoView {
    let location = use_location();
    let filename = move || { location
        .pathname
        .get_untracked()
        .split('/')
        .last()
        .unwrap_or_default()
        .to_string()
    };

    let blog_post = get_blog_post(filename());
    
    let (title_text, desc_text, og_image) = match &blog_post {
        Ok(p) => (
            p.title.clone(),
            p.content
             .lines()
             .map(str::trim)
             .take(10)
             .collect::<Vec<_>>()
             .join(" ")
             .chars().take(150).collect(),
            format!("https://regalk.dev/{}", p.image),
        ),
        Err(_) => (
            "Blog post not found".to_string(),
            "No description available".to_string(),
            "/default-og.png".to_string(),
        ),
    };

    view! {
        <Title text=match &blog_post {
            Ok(p) => p.title.clone(),
            Err(_) => "Blog post not found".to_string(),
        } />
        <Meta name="description" content=desc_text.clone() />
        <Meta property="og:title" content=title_text.clone() />
        <Meta property="og:description" content=desc_text.clone() />
        <Meta property="og:image" content=og_image.clone() />
        <Meta name="twitter:card" content="summary_large_image" />
        <Meta name="twitter:image" content=og_image.clone() />

        <div>
            {match blog_post {
                Ok(post) => {
                    view! {
                        <header class="title-main--page--container">
                            <h1>{post.title.to_string()}</h1>
                        </header>
                        <div class="blog--content-view">
                            <br />
                            <br />

                            <span>{post.date.to_string()}</span>
                            <div class="md-content">
                                <div>{parse_md_content(&post.content.to_string())}</div>
                            </div>
                        </div>
                    }
                        .into_any()
                }
                Err(_e) => {
                    view! {
                        <header class="title-main--page--container">
                            <h1>{"NO BLOG POST FOUND".to_string()}</h1>
                        </header>
                        <div class="blog--content-view">
                            <h2>{"The singularity is nearer".to_string()}</h2>
                            <span>{"2099-99-99".to_string()}</span>
                            <div inner_html="<p></p>".to_string()></div>
                        </div>
                    }
                        .into_any()
                }
            }}
        </div>
    }
}

#[derive(Clone, serde::Serialize, serde::Deserialize, Debug)]
pub struct BlogPost {
    title: String,
    date: String,
    image: String,
    content: String,
    filename: String,
}

#[derive(Error, Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum BlogError {
    #[error("IO error: {0}")]
    Io(String),
    #[error("Blog post not found: {0}")]
    NotFound(String),
}
