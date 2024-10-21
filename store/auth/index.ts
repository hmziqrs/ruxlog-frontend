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
        logg: actions.logg(set),

        logxa: actions.logxa(set),

        logxx: actions.logxx(set),

        logx: actions.logx(set),

        log: actions.log(set),

        login: actions.login(set),

        reset: actions.reset(set)
      }
    };
  })
);
