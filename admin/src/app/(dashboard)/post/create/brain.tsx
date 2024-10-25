import * as z from 'zod';
import { zodResolver } from '@hookform/resolvers/zod';
import { useForm } from 'react-hook-form';
import { usePost } from '@/store/post';
import { useEffect } from 'react';
import { toast } from 'sonner';
import { useBoolEngine, usePrev } from '@/hooks/react-hooks';

const formSchema = z.object({
  title: z.string().min(1, 'Title is required'),
  content: z.string().min(1, 'Content is required'),
  slug: z
    .string()
    .min(1, 'Slug is required')
    .regex(
      // /^[a-z0-9]+(?:-[a-z0-9]+)*$/,
      /^[a-z0-9-_]+$/,
      'Slug can only contain lowercase letters, numbers, and hyphens'
    )
    .transform((value) => value.toLowerCase()),
  excerpt: z.string().nullable(),
  featuredImageUrl: z.string().nullable(),
  isPublished: z.boolean().default(false),
  publishedAt: z.string().nullable(),
  categoryId: z.number().nullable(),
  tagIds: z.array(z.number()).default([]),
});

export type NewPostFormValues = z.infer<typeof formSchema>;

export function useNewPostBrain() {
  const autoSlug = useBoolEngine(true);
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

  const title = form.watch('title');

  async function onSubmit(values: NewPostFormValues) {
    console.log(values);
    // await post.actions.add(values);
  }

  function sanitizeSlug(text: string): string {
    return text
      .toLowerCase() // convert to lowercase
      .replace(/[^\w\s-]/g, '') // remove special characters
      .replace(/\s+/g, '-') // replace spaces with hyphens
      .replace(/-+/g, '-') // replace multiple hyphens with single hyphen
      .replace(/^-+|-+$/g, ''); // remove leading and trailing hyphens
  }

  useEffect(() => {
    if (autoSlug.bool && title) {
      form.setValue('slug', sanitizeSlug(title));
    }
  }, [title, autoSlug, form]);

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
    sanitizeSlug,
    autoSlug,
  };
}
