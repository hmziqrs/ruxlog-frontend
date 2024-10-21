import { create } from 'zustand';
import { immer } from 'zustand/middleware/immer';

import { AuthStore } from './types';
import { authState } from './data';
import * as actions from './actions';

export const useAuth = create(
  immer<AuthStore>((set, get) => {
    return {
      ...authState,
      actions: {
        login: actions.login(set),

        reset: actions.reset(set)
      }
    };
  })
);
