import { useEffect } from 'react';
import { toast } from 'sonner';
import { usePost } from '@/store/post';
import { useDidMount, usePrev } from '@/hooks/react-hooks';

export function usePreviewBrain(postId: number) {
  const posts = usePost();
  const didMount = useDidMount();
  const viewState = posts.state.view[postId];
  const prevViewState = usePrev(viewState);
  const editState = posts.state.edit[postId];
  const prevEditState = usePrev(editState);

  // Try to get post from list first
  const cachedPost = posts.data.list.find((p) => p.id === postId);
  const post = posts.data.view[postId] ?? cachedPost;

  useEffect(() => {
    if (didMount) return;
    posts.actions.view(postId);
  }, [didMount, postId]);

  useEffect(() => {
    if (prevViewState?.loading && !viewState?.loading) {
      if (viewState?.error) {
        toast.error('Failed to load post');
      }
    }
  }, [viewState, prevViewState]);

  useEffect(() => {
    if (prevEditState?.loading && !editState?.loading) {
      if (editState?.success) {
        toast.success('Post updated successfully');
        posts.actions.view(postId);
      } else if (editState?.error) {
        toast.error(editState?.message || 'Failed to update post');
      }
    }
  }, [editState, prevEditState, postId]);

  function handleTogglePublish() {
    if (!post) return;
    posts.actions.edit(postId, { isPublished: !post.isPublished });
  }

  return {
    post,
    loading: viewState?.loading,
    error: viewState?.error,
    handleTogglePublish,
  };
}
