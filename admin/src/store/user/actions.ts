import {ImmerAction, ImmerState} from '@/store/types';
import { mapCatchError } from '@/store/utils';
import {subState} from '@/store/data';
import {api} from '@/services/api';

import {UserStore } from './types';
import {userState} from './data';




export const add = (set: ImmerAction<UserStore>) => async () => {
  set(state => {
    state.state.add = {...subState, loading: true};
  });
  try {
    // Add your API call here
    set(state => {
      state.state.add = {...subState, success: true};
      // Update state.data.add here
    });
  } catch (error) {
    set(state => {
      state.state.add = {
        ...subState,
        error: true,
        message: mapCatchError(error),
      };
    });
  }
};


export const edit = (set: ImmerAction<UserStore>) => async () => {
  set(state => {
    state.state.edit = {...subState, loading: true};
  });
  try {
    // Add your API call here
    set(state => {
      state.state.edit = {...subState, success: true};
      // Update state.data.edit here
    });
  } catch (error) {
    set(state => {
      state.state.edit = {
        ...subState,
        error: true,
        message: mapCatchError(error),
      };
    });
  }
};


export const remove = (set: ImmerAction<UserStore>) => async () => {
  set(state => {
    state.state.remove = {...subState, loading: true};
  });
  try {
    // Add your API call here
    set(state => {
      state.state.remove = {...subState, success: true};
      // Update state.data.remove here
    });
  } catch (error) {
    set(state => {
      state.state.remove = {
        ...subState,
        error: true,
        message: mapCatchError(error),
      };
    });
  }
};


export const list = (set: ImmerAction<UserStore>) => async () => {
  set(state => {
    state.state.list = {...subState, loading: true};
  });
  try {
    // Add your API call here
    set(state => {
      state.state.list = {...subState, success: true};
      // Update state.data.list here
    });
  } catch (error) {
    set(state => {
      state.state.list = {
        ...subState,
        error: true,
        message: mapCatchError(error),
      };
    });
  }
};


export const view = (set: ImmerAction<UserStore>) => async () => {
  set(state => {
    state.state.view = {...subState, loading: true};
  });
  try {
    // Add your API call here
    set(state => {
      state.state.view = {...subState, success: true};
      // Update state.data.view here
    });
  } catch (error) {
    set(state => {
      state.state.view = {
        ...subState,
        error: true,
        message: mapCatchError(error),
      };
    });
  }
};


export const change_password = (set: ImmerAction<UserStore>) => async () => {
  set(state => {
    state.state.change_password = {...subState, loading: true};
  });
  try {
    // Add your API call here
    set(state => {
      state.state.change_password = {...subState, success: true};
      // Update state.data.change_password here
    });
  } catch (error) {
    set(state => {
      state.state.change_password = {
        ...subState,
        error: true,
        message: mapCatchError(error),
      };
    });
  }
};


export const reset = (set: ImmerAction<UserStore>) => async () => {
  set(state => {
    state.state = {...userState.state};
    state.data = {...userState.data};
  });
};
