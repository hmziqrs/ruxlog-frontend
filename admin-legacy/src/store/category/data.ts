import { subState } from '@/store/data';
import { CategoryState } from './types';

export const categoryState: CategoryState = {
  state: {
    add: { ...subState },

    edit: {},

    remove: {},

    list: { ...subState },

    view: {},
  },
  data: {
    add: null,

    edit: null,

    remove: null,

    list: [],

    view: {},
  },
};
