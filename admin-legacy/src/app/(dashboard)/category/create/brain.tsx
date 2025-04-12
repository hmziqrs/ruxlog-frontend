import { useEffect } from 'react';
import { useCategory } from '@/store/category';
import { toast } from 'sonner';
import { usePrev } from '@/hooks/react-hooks';
import { useRouter } from 'next/navigation';

export function useNewCategoryBrain() {
  const category = useCategory();
  const prevAddState = usePrev(category.state.add);
  const router = useRouter();
  const { loading } = category.state.add;

  function onSubmit(data: any) {
    category.actions.add(data);
  }

  useEffect(() => {
    if (prevAddState?.loading && !category.state.add.loading) {
      if (category.state.add.success) {
        router.push('/category');
        category.actions.list();
        toast.success('Category created successfully!');
      } else if (category.state.add.error) {
        toast.error(category.state.add.message ?? 'Failed to create category');
      }
    }
  }, [category.state.add, prevAddState]);

  return {
    loading,
    onSubmit,
  };
}
