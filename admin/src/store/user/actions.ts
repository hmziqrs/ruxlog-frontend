import { ImmerAction, ImmerState } from '@/store/types';
import { mapCatchError } from '@/store/utils';
import { subState } from '@/store/data';
import { api } from '@/services/api';

import { User, UserAddPayload, UserEditPayload, UserStore } from './types';
import { userState } from './data';

export const add =
  (set: ImmerAction<UserStore>) => async (payload: UserAddPayload) => {
    set((state) => {
      state.state.add = { ...subState, loading: true };
    });
    try {
      const res = await api.post<User>('/admin/user/v1/create', payload);
      set((state) => {
        state.state.add = { ...subState, success: true };
        state.data.list = [res.data, ...state.data.list];
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
  (set: ImmerAction<UserStore>) =>
  async (id: number, payload: UserEditPayload) => {
    set((state) => {
      state.state.edit = {
        ...state.state.edit,
        [id]: { ...subState, loading: true },
      };
    });
    try {
      const res = await api.post<User>(`/admin/user/v1/update/${id}`, payload);
      set((state) => {
        state.state.edit[id] = { ...subState, success: true };
        state.data.list = state.data.list.map((user) =>
          user.id === id ? res.data : user
        );
        if (state.data.view[id]) {
          state.data.view[id] = res.data;
        }
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

export const remove = (set: ImmerAction<UserStore>) => async (id: number) => {
  set((state) => {
    state.state.remove = {
      ...state.state.remove,
      [id]: { ...subState, loading: true },
    };
  });
  try {
    await api.post(`/admin/user/v1/delete/${id}`);
    set((state) => {
      state.state.remove[id] = { ...subState, success: true };
      state.data.list = state.data.list.filter((user) => user.id !== id);
      delete state.data.view[id];
    });
  } catch (error) {
    set((state) => {
      state.state.remove[id] = {
        ...subState,
        error: true,
        message: mapCatchError(error),
      };
    });
  }
};

export const list = (set: ImmerAction<UserStore>) => async () => {
  set((state) => {
    state.state.list = { ...subState, loading: true };
  });
  try {
    const res = await api.get<User[]>('/admin/user/v1/list');
    set((state) => {
      state.state.list = { ...subState, success: true };
      state.data.list = res.data;
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

export const view = (set: ImmerAction<UserStore>) => async (id: number) => {
  set((state) => {
    state.state.view = {
      ...state.state.view,
      [id]: { ...subState, loading: true },
    };
  });
  try {
    const res = await api.get<User>(`/admin/user/v1/view/${id}`);
    set((state) => {
      state.state.view[id] = { ...subState, success: true };
      state.data.view[id] = res.data;
    });
  } catch (error) {
    set((state) => {
      state.state.view[id] = {
        ...subState,
        error: true,
        message: mapCatchError(error),
      };
    });
  }
};

export const reset = (set: ImmerAction<UserStore>) => async () => {
  set((state) => {
    state.state = { ...userState.state };
    state.data = { ...userState.data };
  });
};
