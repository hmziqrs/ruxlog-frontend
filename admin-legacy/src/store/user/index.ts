import { create } from 'zustand';
import { immer } from 'zustand/middleware/immer';

import * as actions from './actions';
import { UserStore } from './types';
import { userState } from './data';

export const useUser = create(
  immer<UserStore>((set, get) => {
    return {
      ...userState,
      actions: {
        add: actions.add(set),
        edit: actions.edit(set),
        remove: actions.remove(set),
        list: actions.list(set),
        view: actions.view(set),
        reset: actions.reset(set),
      },
    };
  })
);
