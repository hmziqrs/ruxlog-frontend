import { useEffect, useState } from 'react';
import { toast } from 'sonner';
import { useDidMount, usePrev } from '@/hooks/react-hooks';
import { PostFilters, PostSortBy } from '@/store/post/types';
import { usePost } from '@/store/post';

export type PostBrain = ReturnType<typeof usePostBrain>;

export function usePostBrain() {
  const posts = usePost();
  const [selectedPosts, setSelectedPosts] = useState<number[]>([]);
  const [viewMode, setViewMode] = useState<'grid' | 'list'>('grid');
  // const prevDeleteState = usePrev(posts.state.delete);
  // const prevToggleState = usePrev(posts.state.togglePublish);

  const didMount = useDidMount();

  useEffect(() => {
    if (didMount) return;
    posts.actions.list();
  }, [didMount]);

  // useEffect(() => {
  //   if (prevDeleteState?.loading && !posts.state.delete.loading) {
  //     if (posts.state.delete.success) {
  //       toast.success('Posts deleted successfully!');
  //       posts.actions.fetch(posts.data.filters);
  //     } else if (posts.state.delete.error) {
  //       toast.error(posts.state.delete.message ?? 'Failed to delete posts');
  //     }
  //   }
  // }, [posts.state.delete, prevDeleteState]);

  // useEffect(() => {
  //   if (prevToggleState?.loading && !posts.state.togglePublish.loading) {
  //     if (posts.state.togglePublish.success) {
  //       toast.success('Post status updated successfully!');
  //     } else if (posts.state.togglePublish.error) {
  //       toast.error(
  //         posts.state.togglePublish.message ?? 'Failed to update post status'
  //       );
  //     }
  //   }
  // }, [posts.state.togglePublish, prevToggleState]);

  // const handleSearch = (search: string) => {
  //   const newFilters: PostFilters = { ...posts.data.filters, search };
  //   posts.actions.fetch(newFilters);
  // };

  // const handleSort = (sortBy: PostSortBy) => {
  //   const ascending =
  //     posts.data.filters.sortBy === sortBy
  //       ? !posts.data.filters.ascending
  //       : true;
  //   const newFilters: PostFilters = {
  //     ...posts.data.filters,
  //     sortBy,
  //     ascending,
  //   };
  //   posts.actions.list(newFilters);
  // };

  // const handleTogglePublish = (id: number) => {
  //   posts.actions.togglePublish(id);
  // };

  // const handleDelete = () => {
  //   if (posts.data.selectedPosts.length === 0) return;
  //   posts.actions.delete(posts.data.selectedPosts);
  // };

  return {
    posts: posts.data.list ?? [],
    filters: posts.data.filters,
    loading: posts.state.list.loading,

    viewMode,
    setViewMode,
    selectedPosts,
    setSelectedPosts,
    // handleSearch,
    // handleSort,
    // handleTogglePublish,
    // handleDelete,
    // setViewMode: posts.actions.setViewMode,
    // setSelectedPosts: posts.actions.setSelectedPosts,
  };
}