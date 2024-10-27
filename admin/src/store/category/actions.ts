import { ImmerAction, ImmerState } from '@/store/types';
import { mapCatchError } from '@/store/utils';
import { subState } from '@/store/data';
import { api } from '@/services/api';

import {
  Category,
  CategoryAddPayload,
  CategoryEditPayload,
  CategoryStore,
} from './types';
import { categoryState } from './data';

export const add =
  (set: ImmerAction<CategoryStore>) => async (payload: CategoryAddPayload) => {
    set((state) => {
      state.state.add = { ...subState, loading: true };
    });
    try {
      const res = await api.post<Category>('/category/v1/create', payload);
      set((state) => {
        state.state.add = { ...subState, success: true };
        state.data.view[res.data.id] = res.data;
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
  (set: ImmerAction<CategoryStore>) =>
  async (id: number, payload: CategoryEditPayload) => {
    set((state) => {
      state.state.edit = {
        ...state.state.edit,
        [id]: { ...subState, loading: true },
      };
    });
    try {
      const res = await api.post<Category>(
        `/category/v1/update/${id}`,
        payload
      );
      set((state) => {
        state.state.edit[id] = { ...subState, success: true };
        state.data.view[res.data.id] = res.data;
        state.data.list = state.data.list.map((item) =>
          item.id === res.data.id ? res.data : item
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

export const remove =
  (set: ImmerAction<CategoryStore>) => async (id: number) => {
    set((state) => {
      state.state.remove = {
        ...state.state.remove,
        [id]: { ...subState, loading: true },
      };
    });
    try {
      await api.post(`/category/v1/delete/${id}`);
      set((state) => {
        state.state.remove[id] = { ...subState, success: true };
        state.data.list = state.data.list.filter((item) => item.id !== id);
        state.data.view = {};
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

export const list = (set: ImmerAction<CategoryStore>) => async () => {
  set((state) => {
    state.state.list = { ...subState, loading: true };
  });
  try {
    const res = await api.get<Category[]>('/category/v1/list');
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

export const view = (set: ImmerAction<CategoryStore>) => async (id: number) => {
  set((state) => {
    state.state.view = {
      ...state.state.view,
      [id]: { ...subState, loading: true },
    };
  });
  try {
    const res = await api.get<Category>(`/category/v1/view/${id}`);
    set((state) => {
      state.state.view[id] = { ...subState, success: true };
      state.data.view[res.data.id] = res.data;
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

export const reset = (set: ImmerAction<CategoryStore>) => async () => {
  set((state) => {
    state.state = { ...categoryState.state };
    state.data = { ...categoryState.data };
  });
};