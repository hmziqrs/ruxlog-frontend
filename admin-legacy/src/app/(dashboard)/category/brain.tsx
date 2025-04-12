import { useEffect, useState } from 'react';
import { toast } from 'sonner';
import { useDidMount, usePrev } from '@/hooks/react-hooks';
import { useCategory } from '@/store/category';

export type CategoryBrain = ReturnType<typeof useCategoryBrain>;

export function useCategoryItemBrain(id: number) {
  const categories = useCategory();
  const editState = categories.state.edit[id] ?? {};
  const removeState = categories.state.remove[id] ?? {};
  const prevEditState = usePrev(editState);
  const prevRemoveState = usePrev(removeState);

  function removeCategory() {
    categories.actions.remove(id);
  }

  useEffect(() => {
    if (prevEditState?.loading && !editState.loading) {
      if (editState.success) {
        toast.success('Category updated successfully!');
      } else if (editState.error) {
        toast.error(editState.message ?? 'Failed to update category');
      }
    }
  }, [categories.state.edit, prevEditState]);

  useEffect(() => {
    if (prevRemoveState?.loading && !removeState.loading) {
      if (removeState.success) {
        toast.success('Category deleted successfully!');
      } else if (removeState.error) {
        toast.error(removeState.message ?? 'Failed to delete category');
      }
    }
  }, [categories.state.remove, prevRemoveState]);

  return {
    removeCategory,
    loading: editState.loading || removeState.loading,
  };
}

export function useCategoryBrain() {
  const categories = useCategory();
  const didMount = useDidMount();

  useEffect(() => {
    if (didMount) return;
    categories.actions.list();
  }, [didMount]);

  return {
    categories: categories.data.list ?? [],
    loading: categories.state.list.loading,
  };
}
