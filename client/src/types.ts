export interface Author {
  avatar: string | null;
  email: string;
  id: number;
  name: string;
}

export interface Category {
  id: number;
  name: string;
}

export interface Tag {
  id: number;
  name: string;
}

export interface Post {
  author: Author;
  authorId: number;
  category: Category | null;
  categoryId: number | null;
  content: string;
  createdAt: string;
  excerpt: string;
  featuredImageUrl: string | null;
  id: number;
  isPublished: boolean;
  likesCount: number;
  publishedAt: string | null;
  slug: string;
  tagIds: number[];
  tags: Tag[];
  title: string;
  updatedAt: string;
  viewCount: number;
}

export interface PostSiteMap {
  slug: string;
  updatedAt: string;
  publishedAt: string;
}