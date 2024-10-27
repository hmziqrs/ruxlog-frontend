import {create} from 'zustand';
import {immer} from 'zustand/middleware/immer';

import * as actions from './actions';
import {CategoryStore} from './types';
import {categoryState} from './data';

export const useCategory = create(
  immer<CategoryStore>((set, get) => {
    return {
      ...categoryState,
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
