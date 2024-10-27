import {create} from 'zustand';
import {immer} from 'zustand/middleware/immer';

import * as actions from './actions';
import {TagStore} from './types';
import {tagState} from './data';

export const useTag = create(
  immer<TagStore>((set, get) => {
    return {
      ...tagState,
      actions: {

        add: actions.add(set),

        edit: actions.edit(set),

        remove: actions.remove(set),

        list: actions.list(set),

        view: actions.view(set),

        reset: actions.reset(set),
      },
    };
  }),
);
