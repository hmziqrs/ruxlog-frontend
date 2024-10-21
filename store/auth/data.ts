import { subState } from '@/store/data';
import { AuthState } from './types';

export const authState: AuthState = {
  state: {
    log: { ...subState },

    login: { ...subState }
  },
  data: {
    log: null,

    login: null
  }
};
