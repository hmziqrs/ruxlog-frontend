import { subState } from '@/store/data';
import { UserState } from './types';

export const userState: UserState = {
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
