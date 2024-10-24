import { subState } from '@/store/data';
import { AuthState } from './types';

export const authState: AuthState = {
  state: {
    login: { ...subState },

    logout: { ...subState },

    init: { ...subState, init: false }
  },
  data: {
    login: null,

    logout: null,

    init: null,

    user: null
  }
};
