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
];

export default bookmarks;
