import { create } from 'zustand';
import { immer } from 'zustand/middleware/immer';

import * as actions from './actions';
import { AuthStore } from './types';
import { authState } from './data';

export const useAuth = create(
  immer<AuthStore>((set, get) => {
    return {
      ...authState,
      actions: {
        login: actions.login(set),

        logout: actions.logout(set),

        init: actions.init(set),

        reset: actions.reset(set),
      },
    };
  })
);
