import { Draft } from 'immer';

export interface SubState {
  success: boolean;
  loading: boolean; // since loading will be either true or either false hence boolean
  error: boolean;
  message?: string;
}

export type ImmerAction<T> = (
  nextStateOrUpdater: T | Partial<T> | ((state: Draft<T>) => void),
  shouldReplace?: boolean | undefined
) => void;

export type ImmerState<T> = () => T;

export type EmptyCallback = () => void;

export type EmptyState = undefined | null;

export type EmptyStateDefault = EmptyState & any;

export interface BaseActions {
  reset: () => void;
}
