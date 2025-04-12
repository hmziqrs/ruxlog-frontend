import { useEffect, useState } from 'react';
import { toast } from 'sonner';
import { useDidMount, usePrev } from '@/hooks/react-hooks';
import { useTag } from '@/store/tag';

export type TagBrain = ReturnType<typeof useTagBrain>;

export function useTagItemBrain(id: number) {
  const tags = useTag();
  const editState = tags.state.edit[id] ?? {};
  const removeState = tags.state.remove[id] ?? {};
  const prevEditState = usePrev(editState);
  const prevRemoveState = usePrev(removeState);

  function removeTag() {
    tags.actions.remove(id);
  }

  useEffect(() => {
    if (prevEditState?.loading && !editState.loading) {
      if (editState.success) {
        toast.success('Tag updated successfully!');
      } else if (editState.error) {
        toast.error(editState.message ?? 'Failed to update tag');
      }
    }
  }, [tags.state.edit, prevEditState]);

  useEffect(() => {
    if (prevRemoveState?.loading && !removeState.loading) {
      if (removeState.success) {
        toast.success('Tag deleted successfully!');
      } else if (removeState.error) {
        toast.error(removeState.message ?? 'Failed to delete tag');
      }
    }
  }, [tags.state.remove, prevRemoveState]);

  return {
    removeTag,
    loading: editState.loading || removeState.loading,
  };
}

export function useTagBrain() {
  const tags = useTag();
  const didMount = useDidMount();

  useEffect(() => {
    if (didMount) return;
    tags.actions.list();
  }, [didMount]);

  return {
    tags: tags.data.list ?? [],
    loading: tags.state.list.loading,
  };
}
