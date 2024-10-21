import { subState } from '@/store/data';
import { AuthState } from './types';

export const authState: AuthState = {
  state: {
    logx: { ...subState },

    log: { ...subState },

    login: { ...subState }
  },
  data: {
    logx: null,

    log: null,

    login: null
  }
};
