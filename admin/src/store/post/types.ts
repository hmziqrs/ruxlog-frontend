import {
  BaseActions,
  SubState,
  EmptyCallback,
  EmptyState,
  EmptyStateDefault,
} from '@/store/types';

export interface PostState {
  state: {
    view: SubState;

    list: SubState;

    add: SubState;

    edit: { [id: number]: SubState };

    remove: { [id: number]: SubState };

    bulkRemove: SubState;
  };
  data: {
    view: { [id: number]: Post | null };

    list: Post[];

    add: EmptyStateDefault;

    edit: EmptyStateDefault;

    remove: EmptyStateDefault;

    bulkRemove: EmptyStateDefault;

    filters: PostFilters;
  };
}

export interface PostActions {
  actions: BaseActions & {
    view: (postId: number) => void;

    list: EmptyCallback;

    add: (payload: PostCreatePayload) => void;

    edit: (id: number, payload: PostEditPayload) => void;

    remove: (id: number) => void;

    bulkRemove: EmptyCallback;
  };
}

export interface PostAuthor {
  id: number;
  name: string;
  email: string;
  avatar: string | null;
}

export interface PostCategory {
  id: number;
  name: string;
}

export interface PostTag {
  id: number;
  name: string;
}

export interface Post {
  id: number;
  title: string;
  content: string;
  slug: string;
  excerpt: string | null;
  featuredImageUrl: string | null;
  isPublished: boolean;
  publishedAt: string | null;
  createdAt: string;
  updatedAt: string;
  authorId: number;
  author: PostAuthor;
  categoryId: number | null;
  category: PostCategory | null;
  tagIds: number[];
  tags: PostTag[];
  likesCount: number;
  viewCount: number;
}

export type PostSortBy =
  | 'Title'
  | 'UpdatedAt'
  | 'PublishedAt'
  | 'ViewCount'
  | 'LikesCount';

export interface PostFilters {
  search?: string;
  sortBy?: PostSortBy;
  ascending?: boolean;
}

export interface PostStore extends PostState, PostActions {}

export interface PostCreatePayload {
  title: string;
  content: string;
  slug: string;
  excerpt?: null | string;
  featuredImageUrl?: null | string;
  isPublished?: boolean;
  categoryId?: null | number;
  tagIds?: number[];
}

export interface PostEditPayload extends Partial<PostCreatePayload> {}
