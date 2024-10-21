import {
  BaseActions,
  SubState,
  EmptyCallback,
  EmptyState,
  EmptyDefaultState
} from '@/store/types';

export interface AuthState {
  state: {
    logg: SubState;

    logx: SubState;

    log: SubState;

    login: SubState;
  };
  data: {
    logg: EmptyDefaultState;

    log: EmptyDefaultState;

    logx: EmptyDefaultState;

    login: EmptyDefaultState;
  };
}

export interface AuthActions {
  actions: BaseActions & {
    login: EmptyCallback;
  };
}

export interface AuthStore extends AuthState, AuthActions {}
