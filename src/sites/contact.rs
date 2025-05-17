use leptos::prelude::*;

#[component]
pub fn Contact() -> impl IntoView { 

    return view! {
        <header class="title-main--page--container">
            <h1>CONTACT</h1>
        </header>
        <section class="contact-me--container">
            <div>
                <img src="apu_band.gif" alt="Guys band playing a song" />
            </div>
            <div class="contact-ways--container">
                <h2>Digital Methods</h2>
                <p>
                    "I'm open to chatting about my blog posts, books from my library, ongoing projects, new project ideas, job opportunities, and any technical topics that spark curiosity. Feel free to reach out for interesting discussions!"
                </p>
                <h3>Email <a href="mailto:contact@regalk.dev">contact [at] regalk [dot] dev</a></h3>
                <div class="email--contact-me">
                    <p>
                        To ensure confidentiality and authenticity, I use GPG for email encryption and signing:
                    </p>
                    <ul>
                        <li>
                            Public GPG Key (for encrypting messages to me):
                            <code>
                                "curl -sL https://keys.openpgp.org/vks/v1/by-email/contact%40regalk.dev | gpg --import"
                            </code>
                        </li>
                        <li>"Fingerprint: C118 4893 ABEE 48F5 42DC  F312 272E B240 4D99 8FBF"</li>
                        <li>
                            "(Important) All legitimate emails from me will be signed with my GPG key."
                        </li>
                    </ul>
                </div>
                <h3>Matrix: @regalk:regalk.dev</h3>
            </div>
        </section>
    }
}
