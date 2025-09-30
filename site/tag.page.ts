export const layout = "tag.vto";

export default function* ({ search, bookmarks }: Lume.Data) {
  const tags = new Set<string>();

  const pages = search.pages();
  for (const page of pages) {
    if (page.tags) {
      const pageTags = Array.isArray(page.tags) ? page.tags : [page.tags];
      pageTags.forEach((tag: string) => tag && tags.add(tag));
    }
  }

  const bookmarksList = Array.isArray(bookmarks)
    ? bookmarks
    : bookmarks?.bookmarks || [];
  for (const bookmark of bookmarksList) {
    if (bookmark.tags) {
      const bookmarkTags = Array.isArray(bookmark.tags)
        ? bookmark.tags
        : [bookmark.tags];
      bookmarkTags.forEach((tag: string) => tag && tags.add(tag));
    }
  }

  for (const tag of tags) {
    yield {
      url: `/tag/${tag}/`,
      title: `Tag: ${tag}`,
      tag: tag,
    };
  }
}
