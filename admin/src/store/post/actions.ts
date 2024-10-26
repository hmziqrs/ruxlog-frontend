import { ImmerAction, ImmerState } from '@/store/types';
import { mapCatchError } from '@/store/utils';
import { subState } from '@/store/data';
import { api } from '@/services/api';

import { Post, PostCreatePayload, PostEditPayload, PostStore } from './types';
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

export const add =
  (set: ImmerAction<PostStore>) => async (payload: PostCreatePayload) => {
    set((state) => {
      state.state.add = { ...subState, loading: true };
    });
    try {
      await api.post('/post/v1/create', payload);
      set((state) => {
        state.state.add = { ...subState, success: true };
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

export const edit =
  (set: ImmerAction<PostStore>) =>
  async (id: number, payload: PostEditPayload) => {
    set((state) => {
      state.state.edit = {
        ...state.state.edit,
        [id]: { ...subState, loading: true },
      };
    });
    try {
      const res = await api.post<Post>(`/post/v1/update/${id}`, payload);
      set((state) => {
        state.state.edit[id] = { ...subState, success: true };
        state.data.list = state.data.list.map((item) =>
          item.id === id ? { ...item, ...res.data } : item
        );
      });
    } catch (error) {
      set((state) => {
        state.state.edit[id] = {
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

export const view = (set: ImmerAction<PostStore>) => async (postId: number) => {
  set((state) => {
    state.state.view = { ...subState, loading: true };
  });
  try {
    const res = await api.post<Post>(`/post/v1/view/${postId}`);
    set((state) => {
      state.state.view = { ...subState, success: true };
      state.data.view[postId] = res.data;
    });
  } catch (error) {
    set((state) => {
      state.state.view = { ...subState, error: true };
    });
  }
};

export const reset = (set: ImmerAction<PostStore>) => async () => {
  set((state) => {
    state.state = { ...postState.state };
    state.data = { ...postState.data };
  });
};
