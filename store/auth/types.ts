import { BaseActions, SubState } from '@/store/types';

export interface AuthState {
  state: {
    login: SubState;
  };
  data: {
    login?: any | null;
  };
}

export interface AuthActions {
  actions: BaseActions & {
    login: () => void;
  };
}

export interface AuthStore extends AuthState, AuthActions {}
