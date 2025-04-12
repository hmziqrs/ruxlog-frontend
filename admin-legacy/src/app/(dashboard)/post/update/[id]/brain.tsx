import { useEffect } from 'react';
import { usePost } from '@/store/post';
import { toast } from 'sonner';
import { useBoolEngine, useDidMount, usePrev } from '@/hooks/react-hooks';
import { useRouter } from 'next/navigation';

export function useUpdatePostBrain(postId: number) {
  const posts = usePost();
  const router = useRouter();
  const viewLoaded = useBoolEngine(false);
  const editState = posts.state.edit[postId];
  const prevEditState = usePrev(editState);

  const viewState = posts.state.view[postId];
  const prevViewState = usePrev(viewState);
  const didMount = useDidMount();

  const post = posts.data.view[postId];

  function onSubmit(data: any) {
    posts.actions.edit(postId, data);
  }

  useEffect(() => {
    if (didMount) return;
    posts.actions.view(postId);
  }, [didMount]);

  useEffect(() => {
    if (prevEditState?.loading && !editState?.loading) {
      if (editState?.success) {
        router.back();
        toast.success('Post updated successfully!');
      } else if (editState?.error) {
        toast.error(editState?.message ?? 'Failed to update post');
      }
    }
  }, [editState, prevEditState]);

  useEffect(() => {
    if (prevViewState?.loading && !viewState?.loading) {
      if (viewState?.success) {
        viewLoaded.setTrue();
      } else if (viewState?.error) {
        toast.error(viewState?.message ?? 'Failed to fetch post');
      }
    }
  }, [viewState, prevViewState]);

  return {
    post,
    onSubmit,
    loaded: viewLoaded.bool,
    loading: viewState?.loading,
    error: viewState?.error,
  };
}
