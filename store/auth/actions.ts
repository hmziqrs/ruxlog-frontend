import { ImmerAction, ImmerState } from '@/store/types';
import { subState } from '@/store/data';
import { api, errorMessage } from '@/services/api';
import { AuthStore } from './types';
import { authState } from './data';

export const login = (set: ImmerAction<AuthStore>) => async () => {
  set((state) => {
    state.state.login = { ...subState, loading: true };
  });
  try {
    // Add your API call here
    set((state) => {
      state.state.login = { ...subState, success: true };
      // Update state.data.login here
    });
  } catch (error) {
    set((state) => {
      state.state.login = { ...subState, error: true };
    });
  }
};

export const log = (set: ImmerAction<AuthStore>) => async () => {
  set((state) => {
    state.state.log = { ...subState, loading: true };
  });
  try {
    // Add your API call here
    set((state) => {
      state.state.log = { ...subState, success: true };
      // Update state.data.log here
    });
  } catch (error) {
    set((state) => {
      state.state.log = { ...subState, error: true };
    });
  }
};

export const reset = (set: ImmerAction<AuthStore>) => async () => {
  set((state) => {
    state.state = { ...authState.state };
    state.data = { ...authState.data };
  });
};
