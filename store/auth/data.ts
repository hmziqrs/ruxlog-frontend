import { subState } from '@/store/data';
import { AuthState } from './types';

export const authState: AuthState = {
  state: {
    logg: { ...subState },

    logxa: { ...subState },

    logxx: { ...subState },

    logx: { ...subState },

    log: { ...subState },

    login: { ...subState }
  },
  data: {
    logg: null,

    logxa: null,

    logxx: null,

    logx: null,

    log: null,

    login: null
  }
};
