import { useEffect } from 'react';
import { toast } from 'sonner';
import { useCategory } from '@/store/category';
import { useDidMount, usePrev } from '@/hooks/react-hooks';

export function usePreviewBrain(categoryId: number) {
  const categories = useCategory();
  const didMount = useDidMount();
  const viewState = categories.state.view[categoryId];
  const prevViewState = usePrev(viewState);
  const editState = categories.state.edit[categoryId];
  const prevEditState = usePrev(editState);

  // Try to get category from list first
  const cachedCategory = categories.data.list.find((c) => c.id === categoryId);
  const category = categories.data.view[categoryId] ?? cachedCategory;

  useEffect(() => {
    if (didMount) return;
    categories.actions.view(categoryId);
  }, [didMount, categoryId]);

  useEffect(() => {
    if (prevViewState?.loading && !viewState?.loading) {
      if (viewState?.error) {
        toast.error('Failed to load category');
      }
    }
  }, [viewState, prevViewState]);

  useEffect(() => {
    if (prevEditState?.loading && !editState?.loading) {
      if (editState?.success) {
        toast.success('Category updated successfully');
        categories.actions.view(categoryId);
      } else if (editState?.error) {
        toast.error(editState?.message || 'Failed to update category');
      }
    }
  }, [editState, prevEditState, categoryId]);

  return {
    category,
    loading: viewState?.loading,
    error: viewState?.error,
  };
}
