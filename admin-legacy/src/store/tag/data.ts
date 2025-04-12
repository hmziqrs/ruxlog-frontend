import { subState } from '@/store/data';
import { TagState } from './types';

export const tagState: TagState = {
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
