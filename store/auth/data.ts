import { subState } from '@/store/data';
import { AuthState } from './types';

export const authState: AuthState = {
  state: {
    login: { ...subState }
  },
  data: {
    login: null
  }
};
