import { ImmerAction, ImmerState } from '@/store/types';
import { mapCatchError } from '@/store/utils';
import { subState } from '@/store/data';
import { api } from '@/services/api';

import { Post, PostStore } from './types';
import { postState } from './data';

export const list = (set: ImmerAction<PostStore>) => async () => {
  set((state) => {
    state.state.list = { ...subState, loading: true };
  });
  try {
    const res = await api.post<Post[]>('/post/v1/list/query', {});
    set((state) => {
      state.state.list = { ...subState, success: true };
      state.data.list = res.data;
      // Update state.data.list here
    });
  } catch (error) {
    set((state) => {
      state.state.list = {
        ...subState,
        error: true,
        message: mapCatchError(error),
      };
    });
  }
};

export const add = (set: ImmerAction<PostStore>) => async () => {
  set((state) => {
    state.state.add = { ...subState, loading: true };
  });
  try {
    // Add your API call here
    set((state) => {
      state.state.add = { ...subState, success: true };
      // Update state.data.add here
    });
  } catch (error) {
    set((state) => {
      state.state.add = {
        ...subState,
        error: true,
        message: mapCatchError(error),
      };
    });
  }
};

export const edit = (set: ImmerAction<PostStore>) => async () => {
  set((state) => {
    state.state.edit = { ...subState, loading: true };
  });
  try {
    // Add your API call here
    set((state) => {
      state.state.edit = { ...subState, success: true };
      // Update state.data.edit here
    });
  } catch (error) {
    set((state) => {
      state.state.edit = {
        ...subState,
        error: true,
        message: mapCatchError(error),
      };
    });
  }
};

export const remove = (set: ImmerAction<PostStore>) => async () => {
  set((state) => {
    state.state.remove = { ...subState, loading: true };
  });
  try {
    // Add your API call here
    set((state) => {
      state.state.remove = { ...subState, success: true };
      // Update state.data.remove here
    });
  } catch (error) {
    set((state) => {
      state.state.remove = {
        ...subState,
        error: true,
        message: mapCatchError(error),
      };
    });
  }
};

export const bulkRemove = (set: ImmerAction<PostStore>) => async () => {
  set((state) => {
    state.state.bulkRemove = { ...subState, loading: true };
  });
  try {
    // Add your API call here
    set((state) => {
      state.state.bulkRemove = { ...subState, success: true };
      // Update state.data.bulkRemove here
    });
  } catch (error) {
    set((state) => {
      state.state.bulkRemove = {
        ...subState,
        error: true,
        message: mapCatchError(error),
      };
    });
  }
};

export const reset = (set: ImmerAction<PostStore>) => async () => {
  set((state) => {
    state.state = { ...postState.state };
    state.data = { ...postState.data };
  });
};
