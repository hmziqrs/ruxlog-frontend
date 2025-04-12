import {
  BaseActions,
  SubState,
  EmptyCallback,
  EmptyState,
  EmptyStateDefault,
} from '@/store/types';

export interface TagState {
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

    list: Tag[];

    view: { [id: number]: Tag };
  };
}

export interface TagActions {
  actions: BaseActions & {
    add: (payload: TagAddPayload) => void;

    edit: (id: number, payload: TagEditPayload) => void;

    remove: (id: number) => void;

    list: EmptyCallback;

    view: (id: number) => void;
  };
}

export interface TagStore extends TagState, TagActions {}

export interface Tag {
  id: number;
  name: string;
  slug: string;
  createdAt: string;
  updatedAt: string;
  description: string | null;
}

export interface TagAddPayload
  extends Omit<Tag, 'id' | 'createdAt' | 'updatedAt'> {}

export interface TagEditPayload extends Partial<TagAddPayload> {}
