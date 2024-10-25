import * as z from 'zod';
import { zodResolver } from '@hookform/resolvers/zod';
import { useForm } from 'react-hook-form';
import { usePost } from '@/store/post';
import { useEffect } from 'react';
import { toast } from 'sonner';
import { usePrev } from '@/hooks/react-hooks';

const formSchema = z.object({
  title: z.string().min(1, 'Title is required'),
  content: z.string().min(1, 'Content is required'),
  slug: z.string().min(1, 'Slug is required'),
  excerpt: z.string().nullable(),
  featuredImageUrl: z.string().nullable(),
  isPublished: z.boolean().default(false),
  publishedAt: z.string().nullable(),
  categoryId: z.number().nullable(),
  tagIds: z.array(z.number()).default([]),
});

export type NewPostFormValues = z.infer<typeof formSchema>;

export function useNewPostBrain() {
  const post = usePost();
  const prevAddState = usePrev(post.state.add);

  const form = useForm<NewPostFormValues>({
    resolver: zodResolver(formSchema),
    defaultValues: {
      title: '',
      content: '',
      slug: '',
      excerpt: null,
      featuredImageUrl: null,
      isPublished: false,
      publishedAt: null,
      categoryId: null,
      tagIds: [],
    },
  });

  const { loading } = post.state.add;

  async function onSubmit(values: NewPostFormValues) {
    console.log(values);
    // await post.actions.add(values);
  }

  useEffect(() => {
    if (prevAddState?.loading && !post.state.add.loading) {
      if (post.state.add.success) {
        toast.success('Post created successfully!');
        // Optionally redirect to post list
      } else if (post.state.add.error) {
        toast.error(post.state.add.message ?? 'Failed to create post');
      }
    }
  }, [post.state.add, prevAddState]);

  return {
    form,
    onSubmit,
    loading,
  };
}
