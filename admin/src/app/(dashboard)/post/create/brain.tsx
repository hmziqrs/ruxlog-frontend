import { useEffect } from 'react';
import { usePost } from '@/store/post';
import { toast } from 'sonner';
import { usePrev } from '@/hooks/react-hooks';
import { useRouter } from 'next/navigation';

export function useNewPostBrain() {
  const post = usePost();
  const prevAddState = usePrev(post.state.add);
  const router = useRouter();
  const { loading } = post.state.add;

  function onSubmit(data: any) {
    post.actions.add(data);
  }

  useEffect(() => {
    if (prevAddState?.loading && !post.state.add.loading) {
      console.log('side effecting');
      if (post.state.add.success) {
        router.push('/post');
        post.actions.list();
        toast.success('Post created successfully!');
      } else if (post.state.add.error) {
        toast.error(post.state.add.message ?? 'Failed to create post');
      }
    }
  }, [post.state.add, prevAddState]);

  return {
    loading,
    onSubmit,
  };
}
