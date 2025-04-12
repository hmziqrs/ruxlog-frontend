import {
  BaseActions,
  SubState,
  EmptyCallback,
  EmptyState,
  EmptyStateDefault,
} from '@/store/types';

export interface CategoryState {
  state: {
    add: SubState;

    edit: { [id: number]: SubState };

    remove: { [id: number]: SubState };

    list: SubState;

    view: { [id: number]: SubState };
  };
  data: {
    add: EmptyStateDefault;

    edit: EmptyStateDefault;

    remove: EmptyStateDefault;

    list: Category[];

    view: { [id: number]: Category };
  };
}

export interface CategoryActions {
  actions: BaseActions & {
    add: (payload: CategoryAddPayload) => void;

    edit: (id: number, payload: CategoryEditPayload) => void;

    remove: (id: number) => void;

    list: EmptyCallback;

    view: (id: number) => void;
  };
}

export interface CategoryStore extends CategoryState, CategoryActions {}

export interface Category {
  id: number;
  name: string;
  slug: string;
  createdAt: string;
  updatedAt: string;
  coverImage: string | null;
  description: string | null;
  logoImage: string | null;
  parentId: number | null;
}

export interface CategoryAddPayload
  extends Omit<Category, 'id' | 'createdAt' | 'updatedAt'> {}

export interface CategoryEditPayload extends Partial<CategoryAddPayload> {}
