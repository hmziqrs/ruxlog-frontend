import {
  BaseActions,
  SubState,
  EmptyCallback,
  EmptyState,
  EmptyStateDefault
} from '@/store/types';

export interface AuthState {
  state: {
    login: SubState;

    logout: SubState;

    init: SubState;
  };
  data: {
    login: EmptyStateDefault;

    logout: EmptyStateDefault;

    init: EmptyStateDefault;
  };
}

export interface AuthActions {
  actions: BaseActions & {
    login: EmptyCallback;

    logout: EmptyCallback;

    init: EmptyCallback;
  };
}

export interface AuthStore extends AuthState, AuthActions {}
