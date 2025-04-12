import { useEffect } from 'react';
import { useTag } from '@/store/tag';
import { toast } from 'sonner';
import { useBoolEngine, useDidMount, usePrev } from '@/hooks/react-hooks';
import { useRouter } from 'next/navigation';

export function useUpdateTagBrain(tagId: number) {
  const tags = useTag();
  const router = useRouter();
  const viewLoaded = useBoolEngine(false);
  const editState = tags.state.edit[tagId];
  const prevEditState = usePrev(editState);

  const viewState = tags.state.view[tagId];
  const prevViewState = usePrev(viewState);
  const didMount = useDidMount();

  const tag = tags.data.view[tagId];

  function onSubmit(data: any) {
    tags.actions.edit(tagId, data);
  }

  useEffect(() => {
    if (didMount) return;
    tags.actions.view(tagId);
  }, [didMount]);

  useEffect(() => {
    if (prevEditState?.loading && !editState?.loading) {
      if (editState?.success) {
        router.back();
        toast.success('Tag updated successfully!');
      } else if (editState?.error) {
        toast.error(editState?.message ?? 'Failed to update tag');
      }
    }
  }, [editState, prevEditState]);

  useEffect(() => {
    if (prevViewState?.loading && !viewState?.loading) {
      if (viewState?.success) {
        viewLoaded.setTrue();
      } else if (viewState?.error) {
        toast.error(viewState?.message ?? 'Failed to fetch tag');
      }
    }
  }, [viewState, prevViewState]);

  return {
    tag,
    onSubmit,
    loaded: viewLoaded.bool,
    loading: viewState?.loading,
    error: viewState?.error,
  };
}
