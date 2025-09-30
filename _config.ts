import lume from "lume/mod.ts";

import tailwindcss from "lume/plugins/tailwindcss.ts";
import lightningcss from "lume/plugins/lightningcss.ts";
import date from "lume/plugins/date.ts";
import feed from "lume/plugins/feed.ts";

// import katex from "lume/plugins/katex.ts";
import bookmarksData from "./_data/bookmarks.ts";
import libraryData from "./_data/library.ts";

const site = lume({
  src: "./site",
  server: {
    debugBar: false,
  },
});

site.data("layout", "default.vto");
site.data("site_name", "Regalk's website");
site.data("title", "Regalk's website");
site.data(
  "description",
  "Regalk is a software engineer student, interested in systems programming, embedded systems, mathematics, and various CS topics. Here he shares his work and bookmarks",
);
site.data("author", "regalk");

site.data("projects", [
  {
    name: "Lothorien",
    description:
      "Personal nix flake with configuration for multiple hosts and users.",
    url: "https://github.com/regalk13/lothlorien",
    technologies: ["nix", "dotfiles"],
  },
  {
    name: "Spacewars",
    description: "Game made in rust, using bevy and shader programming.",
    url: "http://github.com/regalk13/spacewars",
    technologies: ["rust", "bevy", "wgpu"],
  },
  {
    name: "UNO.rs",
    description:
      "Website made in rust/wasm to play the UNO game blazingly fast.",
    url: "https://github.com/regalk13/uno-rs",
    technologies: ["rust", "wasm", "leptos"],
  },
  {
    name: "Color mixer",
    description:
      "Rust desktop application to paint and mix colors using real color theory.",
    url: "https://github.com/regalk13/mix-colors",
    technologies: ["rust", "wgpu"],
  },
  {
    name: "Previous personal website",
    description:
      "My old personal website made in leptops and deployed using nix.",
    url: "https://github.com/regalk13/regalk-website",
    technologies: ["rust", "wasm", "leptos"],
  },
]);

site.data("library", libraryData);

site.use(tailwindcss());
site.use(lightningcss());
site.use(date());

site.data("bookmarks", bookmarksData);

site.use(feed({
  output: ["/posts.rss", "/posts.json"],
  query: "type=post",
  info: {
    title: "=site.title",
    description: "=site.description",
  },
  items: {
    title: "=title",
    description: "=excerpt",
  },
}));

/*
site.use(katex({
  options: {
    delimiters: [
      { left: "$$", right: "$$", display: true },
      { left: "\\(", right: "\\)", display: false },
      { left: "\\[", right: "\\]", display: true },
      { left: "$", right: "$", display: false }
    ]
  }
}));
*/

site.add(".");

export default site;
