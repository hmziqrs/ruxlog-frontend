import { useEffect } from 'react';
import { toast } from 'sonner';
import { usePost } from '@/store/post';
import { useDidMount, usePrev } from '@/hooks/react-hooks';
import { Post } from '@/store/post/types';

export function usePreviewBrain(postId: number) {
  const posts = usePost();
  const didMount = useDidMount();
  const prevViewState = usePrev(posts.state.view);
  const prevEditState = usePrev(posts.state.edit);

  // Try to get post from list first
  const cachedPost = posts.data.list.find((p) => p.id === postId);
  const post = posts.data.view[postId] ?? cachedPost;

  useEffect(() => {
    if (didMount) return;
    posts.actions.view(postId);
  }, [didMount, postId]);

  useEffect(() => {
    if (prevViewState?.loading && !posts.state.view.loading) {
      if (posts.state.view.error) {
        toast.error('Failed to load post');
      }
    }
  }, [posts.state.view, prevViewState]);

  useEffect(() => {
    if (prevEditState?.loading && !posts.state.edit.loading) {
      if (posts.state.edit.success) {
        toast.success('Post updated successfully');
        posts.actions.view(postId); // Refresh the post data
      } else if (posts.state.edit.error) {
        toast.error(posts.state.edit.message || 'Failed to update post');
      }
    }
  }, [posts.state.edit, prevEditState, postId]);

  const handleTogglePublish = async () => {
    if (!post) return;
    // Implement toggle publish logic here
    // posts.actions.edit({ ...post, isPublished: !post.isPublished });
  };

  return {
    post,
    loading: posts.state.view.loading,
    error: posts.state.view.error,
    handleTogglePublish,
  };
}
