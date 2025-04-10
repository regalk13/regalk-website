use leptos::prelude::*;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer>
            <div class="footer-container">
                <div class="footer-main">
                    <div class="footer-section">
                        <h3>"Contact"</h3>
                        <ul>
                            <li>
                                <a href="mailto:contact@regalk.dev">"Email"</a>
                            </li>
                            <li>
                                <a href="https://github.com/regalk13">"GitHub"</a>
                            </li>
                        </ul>
                    </div>
                    <div class="footer-section">
                        <h3>"Quick Links"</h3>
                        <ul>
                            <li>
                                <a href="#about-me">"About"</a>
                            </li>
                            <li>
                                <a href="#projects">"Projects"</a>
                            </li>
                            <li>
                                <a href="#blog">"Blog"</a>
                            </li>
                        </ul>
                    </div>
                    <div class="footer-section">
                        <h3>"RSS Feed"</h3>
                        <p>
                            "Subscribe to my "<a target="_blank" href="/rss.xml">
                                "RSS feed"
                            </a>
                        </p>
                    </div>
                </div>
                <div class="footer-bottom">
                    <p>"© 2025 Regalk - Built with Rust & ❤️"</p>
                    <p>
                        "This site is open source - "
                        <a href="https://github.com/regalk13/regalk-website">"view source"</a>
                    </p>
                </div>
            </div>
        </footer>
    }
}