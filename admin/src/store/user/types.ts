import {BaseActions, SubState, EmptyCallback, EmptyState, EmptyStateDefault} from '@/store/types';

export interface UserState {
  state: {

    add: SubState;

    edit: SubState;

    remove: SubState;

    list: SubState;

    view: SubState;

    change_password: SubState;

  };
  data: {

    add: EmptyStateDefault;

    edit: EmptyStateDefault;

    remove: EmptyStateDefault;

    list: EmptyStateDefault;

    view: EmptyStateDefault;

    change_password: EmptyStateDefault;

  };
}

export interface UserActions {
  actions: BaseActions & {

    add: EmptyCallback;

    edit: EmptyCallback;

    remove: EmptyCallback;

    list: EmptyCallback;

    view: EmptyCallback;

    change_password: EmptyCallback;

  };
}

export interface UserStore extends UserState, UserActions {}
