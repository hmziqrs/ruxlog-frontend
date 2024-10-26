import { subState } from '@/store/data';
import { PostState } from './types';

export const postState: PostState = {
  state: {
    view: { ...subState },

    list: { ...subState },

    add: { ...subState },

    edit: { ...subState },

    remove: { ...subState },

    bulkRemove: { ...subState },
  },
  data: {
    view: {},

    list: [],

    add: null,

    edit: null,

    remove: null,

    bulkRemove: null,

    filters: {},
  },
};
