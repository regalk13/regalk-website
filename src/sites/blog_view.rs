use leptos::prelude::*;
use leptos_router::hooks::use_location;
use thiserror::Error;
use syntect::easy::HighlightLines;
use syntect::highlighting::Style;
use syntect::parsing::SyntaxSet;
use syntect::util::LinesWithEndings;
use two_face::re_exports::syntect;
use two_face::theme::EmbeddedLazyThemeSet;

include!(concat!(env!("OUT_DIR"), "/blog_posts.rs"));


lazy_static::lazy_static! {
    static ref SYNTAXES: SyntaxSet = two_face::syntax::extra_newlines();
    static ref THEMES: EmbeddedLazyThemeSet = two_face::theme::extra();
    static ref THEME: syntect::highlighting::Theme = THEMES.get(two_face::theme::EmbeddedThemeName::VisualStudioDarkPlus).clone();
}

fn highlight_code_block(code: &str, language: &str) -> String {
    let syntax = SYNTAXES
        .find_syntax_by_token(language)
        .unwrap_or_else(|| SYNTAXES.find_syntax_plain_text());

    let mut h = HighlightLines::new(syntax, &THEME);
    let mut html = String::from("<pre class=\"code-block\"><code>");

    for line in LinesWithEndings::from(code) {
        let ranges: Vec<(Style, &str)> = h.highlight_line(line, &SYNTAXES).unwrap();
        let escaped = syntect::html::styled_line_to_highlighted_html(
            &ranges[..],
            syntect::html::IncludeBackground::No,
        ).unwrap();
        html.push_str(&escaped);
    }

    html.push_str("</code></pre>");
    html
}

#[server]
async fn render_html(code: String, language: String) -> Result<String, ServerFnError> {
    let syntax = SYNTAXES
        .find_syntax_by_token(&language)
        .unwrap_or_else(|| SYNTAXES.find_syntax_plain_text());

    let mut h = HighlightLines::new(syntax, &THEME);
    let mut html = String::from("<pre class=\"code-block\"><code>");

    for line in LinesWithEndings::from(&code) {
        let ranges: Vec<(Style, &str)> = h.highlight_line(line, &SYNTAXES).unwrap();
        let escaped = syntect::html::styled_line_to_highlighted_html(
            &ranges[..],
            syntect::html::IncludeBackground::No,
        ).unwrap();
        html.push_str(&escaped);
    }

    html.push_str("</code></pre>");
    Ok(html)
}


#[component]
fn CodeBlock(code: String, lang: String) -> impl IntoView {
    let html = Resource::new(move || (code.clone(), lang.clone()), |(code, language)| async move {
        render_html(code, language).await
    });

    view! {
        <Suspense fallback=move || {
            view! {
                <pre class="code-block"><code>{""}</code></pre>
            }
        }>
            {move || {
                match html.get() {
                    // still loading: we’ll never hit this because fallback covered it
                    None => view! { <></> }.into_any(),
                    // got HTML
                    Some(Ok(html)) => view! {
                        <div inner_html=html />
                    }.into_any(),
                    Some(Err(_e)) => view! {
                        <></>
                    }.into_any()
                }
            }}
        </Suspense>
    }
}

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

    // Split content into lines. We no longer filter out empty lines because
    // an empty line might be meaningful inside a code block.
    let lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();

    let mut output = Vec::new();
    let mut in_code_block = false;
    let mut code_block_buffer = Vec::new();
    let mut code_lang       = String::new();
    for line in lines {

        if line.trim_start().starts_with("```") {
    if in_code_block { // Ending a code block
        let code = code_block_buffer.join("\n");
        let lang = code_lang.clone();
        output.push(
            view! { <CodeBlock code lang/> }.into_any()
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

    view! {
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
