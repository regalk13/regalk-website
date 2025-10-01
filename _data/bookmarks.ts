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
    url: "https://youtube.com/playlist?list=PLZHQObOWTQDPD3MizzM2xVFitgF8hE_ab&si=42GCg-0-wNWc6t2t",
    description: "A free course offering the core concept of linear algebra with a visuals-first approach.",
    category: "videos",
    tags: ["linear-algebra", "vectors", "math-matrix"],
    date: new Date("2016-08-06"),
    author: "3Blue1Brown"
  }
];

export default bookmarks;
