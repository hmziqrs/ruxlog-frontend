import { ImmerAction, ImmerState } from '@/store/types';
import { subState } from '@/store/data';
import { api } from '@/services/api';

import { AuthLoginPayload, AuthStore } from './types';
import { authState } from './data';

export const login =
  (set: ImmerAction<AuthStore>) => async (payload: AuthLoginPayload) => {
    set((state) => {
      state.state.login = { ...subState, loading: true };
    });
    try {
      const res = await api.post('/auth/v1/log_in', payload);
      const data = res.data;
      console.log('DATA', data);
      set((state) => {
        state.state.login = { ...subState, success: true };
        state.data.user = data;
        // Update state.data.login here
      });
    } catch (error) {
      console.log('ERROR', error);
      set((state) => {
        state.state.login = { ...subState, error: true };
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

export const init = (set: ImmerAction<AuthStore>) => async () => {
  set((state) => {
    state.state.init = { ...subState, loading: true };
  });
  try {
    // Add your API call here
    set((state) => {
      state.state.init = { ...subState, success: true };
      // Update state.data.init here
    });
  } catch (error) {
    set((state) => {
      state.state.init = { ...subState, error: true };
    });
  }
};

export const reset = (set: ImmerAction<AuthStore>) => async () => {
  set((state) => {
    state.state = { ...authState.state };
    state.data = { ...authState.data };
  });
};
