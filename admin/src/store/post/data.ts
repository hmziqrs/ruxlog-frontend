import {subState} from '@/store/data';
import {PostState} from './types';

export const postState: PostState = {
  state: {

    list: {...subState},

    add: {...subState},

    edit: {...subState},

    remove: {...subState},

    bulkRemove: {...subState},

  },
  data: {

    list: null,

    add: null,

    edit: null,

    remove: null,

    bulkRemove: null,

  },
};
