import { useEffect, useState } from 'react';
import { toast } from 'sonner';
import { useDidMount, usePrev } from '@/hooks/react-hooks';
import { useUser } from '@/store/user';

export type UserBrain = ReturnType<typeof useUserBrain>;

export function useUserItemBrain(id: number) {
  const users = useUser();
  const editState = users.state.edit[id] ?? {};
  const removeState = users.state.remove[id] ?? {};
  const prevEditState = usePrev(editState);
  const prevRemoveState = usePrev(removeState);

  function removeUser() {
    users.actions.remove(id);
  }

  useEffect(() => {
    if (prevEditState?.loading && !editState.loading) {
      if (editState.success) {
        toast.success('User updated successfully!');
      } else if (editState.error) {
        toast.error(editState.message ?? 'Failed to update user');
      }
    }
  }, [users.state.edit, prevEditState]);

  useEffect(() => {
    if (prevRemoveState?.loading && !removeState.loading) {
      if (removeState.success) {
        toast.success('User deleted successfully!');
      } else if (removeState.error) {
        toast.error(removeState.message ?? 'Failed to delete user');
      }
    }
  }, [users.state.remove, prevRemoveState]);

  return {
    removeUser,
    loading: editState.loading || removeState.loading,
  };
}

export function useUserBrain() {
  const users = useUser();
  const didMount = useDidMount();

  useEffect(() => {
    if (didMount) return;
    users.actions.list();
  }, [didMount]);

  return {
    users: users.data.list ?? [],
    loading: users.state.list.loading,
  };
}
