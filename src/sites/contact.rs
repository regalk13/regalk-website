use leptos::prelude::*;

#[component]
pub fn Contact() -> impl IntoView { 

    return view! {
        <header class="title-main--page--container">
            <h1>CONTACT ME</h1>
        </header>
        <section class="contact-me--container">
            <h2>The Digital Way</h2>
            <img src="apu_band.gif" alt="Guys band playing a song" />

            <div class="email--contact-me">
                <br />
                <br />

                <ul>
                    <li>
                        <a href="mailto:contact@regalk.dev">contact@regalk.dev</a>
                    </li>
                    <li>GPG key for encrypting mail:</li>
                    <li>
                        "curl -sL https://keys.openpgp.org/vks/v1/by-email/contact%40regalk.dev | gpg --import"
                    </li>
                    <li>"Fingerprint: 18A0 C1AF 6930 7363 9B31  B6E2 8F4A 1ADC E194 1807"</li>
                    <li>"All legitimate emails from me will be signed with my GPG key."</li>
                    <li>"(The email is ready! 🎉)."</li>
                </ul>
                <br />
                <br />
                <p>
                    "I'm open to chatting about my blog posts, books from my library, ongoing projects, new project ideas, job opportunities, and any technical topics that spark curiosity. Feel free to reach out for interesting discussions!"
                </p>
            </div>

        </section>
    }
}
