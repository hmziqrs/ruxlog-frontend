import { ImmerAction, ImmerState } from '@/store/types';
import { mapCatchError } from '@/store/utils';
import { User } from '@/store/user/types';
import { subState } from '@/store/data';
import { api } from '@/services/api';

import { AuthLoginPayload, AuthStore } from './types';
import { authState } from './data';
import * as utils from './utils';

export const init = (set: ImmerAction<AuthStore>) => async () => {
  set((state) => {
    state.state.init = { ...subState, loading: true, init: true };
  });
  try {
    if (!utils.checkIdCookieExist()) {
      set((state) => {
        state.state.init = { ...subState, init: true };
      });
      return;
    }
    const res = await api.get<User>('/user/v1/get');
    if (!res.data.isVerified || res.data.role === 'user') {
      utils.deleteIdCookie();
      throw new Error('User not allowed to access this page.');
    }
    set((state) => {
      state.state.init = { ...subState, success: true, init: true };
      state.data.user = res.data;
      // Update state.data.init here
    });
  } catch (error) {
    set((state) => {
      state.state.init = {
        ...subState,
        error: true,
        init: true,
        message: mapCatchError(error),
      };
    });
  }
};

export const login =
  (set: ImmerAction<AuthStore>) => async (payload: AuthLoginPayload) => {
    set((state) => {
      state.state.login = { ...subState, loading: true };
    });
    try {
      const res = await api.post<User>('/auth/v1/log_in', payload);
      const data = res.data;

      if (!res.data.isVerified || res.data.role === 'user') {
        utils.deleteIdCookie();
        throw new Error('User not allowed to access this page.');
      }

      set((state) => {
        state.state.login = { ...subState, success: true };
        state.data.user = data;
      });
    } catch (error) {
      set((state) => {
        state.state.login = {
          ...subState,
          error: true,
          message: mapCatchError(error, 'Error while trying to login'),
        };
      });
    }
  };

export const logout = (set: ImmerAction<AuthStore>) => async () => {
  set((state) => {
    state.state.logout = { ...subState, loading: true };
  });
  try {
    // Add your API call here
    set((state) => {
      state.state.logout = { ...subState, success: true };
      // Update state.data.logout here
    });
  } catch (error) {
    set((state) => {
      state.state.logout = { ...subState, error: true };
    });
  }
};

export const reset = (set: ImmerAction<AuthStore>) => async () => {
  set((state) => {
    state.state = { ...authState.state };
    state.data = { ...authState.data };
  });
};
