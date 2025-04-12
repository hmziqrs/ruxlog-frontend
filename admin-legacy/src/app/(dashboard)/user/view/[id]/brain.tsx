import { useEffect } from 'react';
import { toast } from 'sonner';
import { useUser } from '@/store/user';
import { useDidMount, usePrev } from '@/hooks/react-hooks';

export function usePreviewBrain(userId: number) {
  const users = useUser();
  const didMount = useDidMount();
  const viewState = users.state.view[userId];
  const prevViewState = usePrev(viewState);
  const editState = users.state.edit[userId];
  const prevEditState = usePrev(editState);

  // Try to get user from list first
  const cachedUser = users.data.list.find((u) => u.id === userId);
  const user = users.data.view[userId] ?? cachedUser;

  useEffect(() => {
    if (didMount) return;
    users.actions.view(userId);
  }, [didMount, userId]);

  useEffect(() => {
    if (prevViewState?.loading && !viewState?.loading) {
      if (viewState?.error) {
        toast.error('Failed to load user');
      }
    }
  }, [viewState, prevViewState]);

  useEffect(() => {
    if (prevEditState?.loading && !editState?.loading) {
      if (editState?.success) {
        toast.success('User updated successfully');
        users.actions.view(userId);
      } else if (editState?.error) {
        toast.error(editState?.message || 'Failed to update user');
      }
    }
  }, [editState, prevEditState, userId]);

  return {
    user,
    loading: viewState?.loading,
    error: viewState?.error,
  };
}
