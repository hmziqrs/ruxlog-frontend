import { useEffect } from 'react';
import { useCategory } from '@/store/category';
import { toast } from 'sonner';
import { useBoolEngine, useDidMount, usePrev } from '@/hooks/react-hooks';
import { useRouter } from 'next/navigation';

export function useUpdateCategoryBrain(categoryId: number) {
  const categories = useCategory();
  const router = useRouter();
  const viewLoaded = useBoolEngine(false);
  const editState = categories.state.edit[categoryId];
  const prevEditState = usePrev(editState);

  const viewState = categories.state.view[categoryId];
  const prevViewState = usePrev(viewState);
  const didMount = useDidMount();

  const category = categories.data.view[categoryId];

  function onSubmit(data: any) {
    categories.actions.edit(categoryId, data);
  }

  useEffect(() => {
    if (didMount) return;
    categories.actions.view(categoryId);
  }, [didMount]);

  useEffect(() => {
    if (prevEditState?.loading && !editState?.loading) {
      if (editState?.success) {
        router.back();
        toast.success('Category updated successfully!');
      } else if (editState?.error) {
        toast.error(editState?.message ?? 'Failed to update category');
      }
    }
  }, [editState, prevEditState]);

  useEffect(() => {
    if (prevViewState?.loading && !viewState?.loading) {
      if (viewState?.success) {
        viewLoaded.setTrue();
      } else if (viewState?.error) {
        toast.error(viewState?.message ?? 'Failed to fetch category');
      }
    }
  }, [viewState, prevViewState]);

  return {
    category,
    onSubmit,
    loaded: viewLoaded.bool,
    loading: viewState?.loading,
    error: viewState?.error,
  };
}
