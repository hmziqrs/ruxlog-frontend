import {
  BaseActions,
  SubState,
  EmptyCallback,
  EmptyState,
  EmptyStateDefault,
} from '@/store/types';

export interface UserState {
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
    list: User[];
    view: { [id: number]: User | null };
  };
}

export interface UserActions {
  actions: BaseActions & {
    add: (payload: UserAddPayload) => void;
    edit: (id: number, payload: UserEditPayload) => void;
    remove: (id: number) => void;
    list: EmptyCallback;
    view: (id: number) => void;
  };
}

export interface UserStore extends UserState, UserActions {}

export type UserRoles =
  | 'super-admin'
  | 'admin'
  | 'moderator'
  | 'author'
  | 'user';

export interface User {
  avatar: string | null;
  createdAt: string;
  email: string;
  id: number;
  isVerified: boolean;
  name: string;
  role: UserRoles;
  updatedAt: string;
}

export interface UserAddPayload
  extends Omit<User, 'id' | 'created_at' | 'updated_at'> {}

export interface UserEditPayload extends Partial<UserAddPayload> {}
