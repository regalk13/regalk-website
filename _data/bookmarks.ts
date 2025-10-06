export interface Bookmark {
  id: string;
  title: string;
  url: string;
  description?: string;
  category: "papers" | "books" | "videos" | "posts" | "other";
  tags: string[];
  date: Date;
  author?: string;
  notes?: string;
}

export const bookmarks: Bookmark[] = [
  {
    id: "1",
    title: "Attention Is All You Need",
    url: "https://arxiv.org/abs/1706.03762",
    description: "The paper that introduced the Transformer architecture",
    category: "papers",
    tags: ["ai", "transformers", "deep-learning", "learning"],
    date: new Date("2024-01-15"),
    author: "Vaswani et al.",
  },
  {
    id: "2",
    title: "Essence of linear algebra",
    url:
      "https://youtube.com/playlist?list=PLZHQObOWTQDPD3MizzM2xVFitgF8hE_ab&si=42GCg-0-wNWc6t2t",
    description:
      "A free course offering the core concept of linear algebra with a visuals-first approach",
    category: "videos",
    tags: ["linear-algebra", "vectors", "math-matrix"],
    date: new Date("2016-08-06"),
    author: "3Blue1Brown",
  },
  {
    id: "3",
    title: "Book of Proof",
    url: "https://richardhammack.github.io/BookOfProof/",
    description:
      "This book is an introduction to the standard methods of proving mathematical theorems",
    category: "books",
    tags: ["discrete-mathematics", "proofs"],
    date: new Date("2018-10-01"),
    author: "Richard Hammack",
  },
  {
    id: "4",
    title: "MIT 6.042J Mathematics for Computer Science",
    url: "https://www.youtube.com/playlist?list=PLB7540DEDD482705B",
    description:
      "This course covers elementary discrete mathematics. Mathematical definitions and proofs are emphasized. Topics include formal logic, induction, graph theory, asymptotic notation and growth of functions, counting principles, and discrete probability",
    category: "videos",
    tags: ["discrete-mathematics", "proofs", "cs"],
    date: new Date("2014-07-02"),
    author: "MIT OpenCourseWare",
  },
  {
    id: "5",
    title: "Bloom’s 3 Stages of Talent Development",
    url: "https://www.justinmath.com/blooms-3-stages-of-talent-development/",
    description:
      " First, fun and exciting playtime. Then, intense and strenuous skill development. Finally, developing one's individual style while pushing the boundaries of the field.",
    category: "posts",
    tags: ["learning", "learning-method"],
    date: new Date("2024-04-30"),
    author: "Justin Skycak (@justinskycak)",
  },
  {
    id: "6",
    title: "The Computer from Pascal to von Neumann",
    url:
      "https://monoskop.org/images/f/fc/Goldstine_Herman_H_The_Computer_from_Pascal_to_von_Neumann.pdf",
    description: "",
    category: "books",
    tags: ["cs", "computer-architecture"],
    date: new Date("1972-01-01"),
    author: "Herman H. Goldstine",
  },
];

export default bookmarks;
