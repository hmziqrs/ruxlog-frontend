import {
  BaseActions,
  SubState,
  EmptyCallback,
  EmptyState,
  EmptyStateDefault,
} from '@/store/types';

export interface PostState {
  state: {
    list: SubState;

    add: SubState;

    edit: SubState;

    remove: SubState;

    bulkRemove: SubState;
  };
  data: {
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
    list: EmptyCallback;

    add: EmptyCallback;

    edit: EmptyCallback;

    remove: EmptyCallback;

    bulkRemove: EmptyCallback;
  };
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
  categoryId: number | null;
  tagIds: number[];
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