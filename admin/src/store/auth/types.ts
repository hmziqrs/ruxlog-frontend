import {
  BaseActions,
  SubState,
  EmptyCallback,
  EmptyState,
  EmptyStateDefault,
} from '@/store/types';
import { User } from '@/store/user/types';

export interface AuthState {
  state: {
    login: SubState;

    logout: SubState;

    init: SubState & { init: boolean };
  };
  data: {
    login: EmptyStateDefault;

    logout: EmptyStateDefault;

    init: EmptyStateDefault;

    user: User | null;
  };
}

export interface AuthActions {
  actions: BaseActions & {
    login: (payload: AuthLoginPayload) => void;

    logout: EmptyCallback;

    init: EmptyCallback;
  };
}

export interface AuthStore extends AuthState, AuthActions {}

export interface AuthLoginPayload {
  email: string;
  password: string;
}
