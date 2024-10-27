import { useEffect } from 'react';
import { useTag } from '@/store/tag';
import { toast } from 'sonner';
import { usePrev } from '@/hooks/react-hooks';
import { useRouter } from 'next/navigation';

export function useNewTagBrain() {
  const tag = useTag();
  const prevAddState = usePrev(tag.state.add);
  const router = useRouter();
  const { loading } = tag.state.add;

  function onSubmit(data: any) {
    tag.actions.add(data);
  }

  useEffect(() => {
    if (prevAddState?.loading && !tag.state.add.loading) {
      if (tag.state.add.success) {
        router.push('/tag');
        tag.actions.list();
        toast.success('Tag created successfully!');
      } else if (tag.state.add.error) {
        toast.error(tag.state.add.message ?? 'Failed to create tag');
      }
    }
  }, [tag.state.add, prevAddState]);

  return {
    loading,
    onSubmit,
  };
}
