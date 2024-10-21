import {
  BaseActions,
  SubState,
  EmptyCallback,
  EmptyState,
  EmptyStateDefault
} from '@/store/types';

export interface AuthState {
  state: {
    logx: SubState;

    log: SubState;

    login: SubState;
  };
  data: {
    logx: EmptyStateDefault;

    log: EmptyStateDefault;

    login: EmptyStateDefault;
  };
}

export interface AuthActions {
  actions: BaseActions & {
    logx: EmptyCallback;

    log: EmptyCallback;

    login: EmptyCallback;
  };
}

export interface AuthStore extends AuthState, AuthActions {}
