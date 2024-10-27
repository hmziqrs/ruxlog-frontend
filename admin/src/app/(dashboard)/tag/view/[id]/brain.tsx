import { useEffect } from 'react';
import { toast } from 'sonner';
import { useTag } from '@/store/tag';
import { useDidMount, usePrev } from '@/hooks/react-hooks';

export function usePreviewBrain(tagId: number) {
  const tags = useTag();
  const didMount = useDidMount();
  const viewState = tags.state.view[tagId];
  const prevViewState = usePrev(viewState);
  const editState = tags.state.edit[tagId];
  const prevEditState = usePrev(editState);

  // Try to get tag from list first
  const cachedTag = tags.data.list.find((t) => t.id === tagId);
  const tag = tags.data.view[tagId] ?? cachedTag;

  useEffect(() => {
    if (didMount) return;
    tags.actions.view(tagId);
  }, [didMount, tagId]);

  useEffect(() => {
    if (prevViewState?.loading && !viewState?.loading) {
      if (viewState?.error) {
        toast.error('Failed to load tag');
      }
    }
  }, [viewState, prevViewState]);

  useEffect(() => {
    if (prevEditState?.loading && !editState?.loading) {
      if (editState?.success) {
        toast.success('Tag updated successfully');
        tags.actions.view(tagId);
      } else if (editState?.error) {
        toast.error(editState?.message || 'Failed to update tag');
      }
    }
  }, [editState, prevEditState, tagId]);

  return {
    tag,
    loading: viewState?.loading,
    error: viewState?.error,
  };
}
