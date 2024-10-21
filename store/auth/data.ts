import { subState } from '@/store/data';
import { AuthState } from './types';

export const authState: AuthState = {
  state: {
    logxa: { ...subState },

    logxx: { ...subState },

    logx: { ...subState },

    log: { ...subState },

    login: { ...subState }
  },
  data: {
    logxa: null,

    logxx: null,

    logx: null,

    log: null,

    login: null
  }
};
