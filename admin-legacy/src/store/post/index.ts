import {create} from 'zustand';
import {immer} from 'zustand/middleware/immer';

import * as actions from './actions';
import {PostStore} from './types';
import {postState} from './data';

export const usePost = create(
  immer<PostStore>((set, get) => {
    return {
      ...postState,
      actions: {
        view: actions.view(set),


        list: actions.list(set),

        add: actions.add(set),

        edit: actions.edit(set),

        remove: actions.remove(set),

        bulkRemove: actions.bulkRemove(set),

        reset: actions.reset(set),
      },
    };
  }),
);
