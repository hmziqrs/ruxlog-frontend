import {subState} from '@/store/data';
import {UserState} from './types';

export const userState: UserState = {
  state: {

    add: {...subState},

    edit: {...subState},

    remove: {...subState},

    list: {...subState},

    view: {...subState},

    change_password: {...subState},

  },
  data: {

    add: null,

    edit: null,

    remove: null,

    list: null,

    view: null,

    change_password: null,

  },
};
