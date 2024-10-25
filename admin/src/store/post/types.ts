import {BaseActions, SubState, EmptyCallback, EmptyState, EmptyStateDefault} from '@/store/types';

export interface PostState {
  state: {

    list: SubState;

    add: SubState;

    edit: SubState;

    remove: SubState;

    bulkRemove: SubState;

  };
  data: {

    list: EmptyStateDefault;

    add: EmptyStateDefault;

    edit: EmptyStateDefault;

    remove: EmptyStateDefault;

    bulkRemove: EmptyStateDefault;

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

export interface PostStore extends PostState, PostActions {}
