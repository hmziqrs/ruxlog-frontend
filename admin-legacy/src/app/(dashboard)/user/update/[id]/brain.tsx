import { useEffect } from 'react';
import { useUser } from '@/store/user';
import { toast } from 'sonner';
import { useBoolEngine, useDidMount, usePrev } from '@/hooks/react-hooks';
import { useRouter } from 'next/navigation';

export function useUpdateUserBrain(userId: number) {
  const users = useUser();
  const router = useRouter();
  const viewLoaded = useBoolEngine(false);
  const editState = users.state.edit[userId];
  const prevEditState = usePrev(editState);

  const viewState = users.state.view[userId];
  const prevViewState = usePrev(viewState);
  const didMount = useDidMount();

  const user = users.data.view[userId];

  function onSubmit(data: any) {
    users.actions.edit(userId, data);
  }

  useEffect(() => {
    if (didMount) return;
    users.actions.view(userId);
  }, [didMount]);

  useEffect(() => {
    if (prevEditState?.loading && !editState?.loading) {
      if (editState?.success) {
        router.back();
        toast.success('User updated successfully!');
      } else if (editState?.error) {
        toast.error(editState?.message ?? 'Failed to update user');
      }
    }
  }, [editState, prevEditState]);

  useEffect(() => {
    if (prevViewState?.loading && !viewState?.loading) {
      if (viewState?.success) {
        viewLoaded.setTrue();
      } else if (viewState?.error) {
        toast.error(viewState?.message ?? 'Failed to fetch user');
      }
    }
  }, [viewState, prevViewState]);

  return {
    user,
    onSubmit,
    loaded: viewLoaded.bool,
    loading: viewState?.loading,
    error: viewState?.error,
  };
}
