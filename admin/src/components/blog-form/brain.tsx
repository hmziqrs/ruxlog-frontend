import * as z from 'zod';
import { zodResolver } from '@hookform/resolvers/zod';
import { useForm } from 'react-hook-form';
import { useEffect } from 'react';
import { useBoolEngine } from '@/hooks/react-hooks';

const formSchema = z.object({
  title: z.string().min(1, 'Title is required'),
  content: z.string().min(1, 'Content is required'),
  slug: z
    .string()
    .min(1, 'Slug is required')
    .regex(
      /^[a-z0-9-_]+$/,
      'Slug can only contain lowercase letters, numbers, and hyphens'
    )
    .transform((value) => value.toLowerCase()),
  excerpt: z.string().nullable(),
  featuredImageUrl: z.string().nullable(),
  isPublished: z.boolean().default(false),
  categoryId: z.number().nullable(),
  tagIds: z.array(z.number()).default([]),
});

export type BlogFormValues = z.infer<typeof formSchema>;

const _defaultValues: BlogFormValues = {
  title: '',
  content: '',
  slug: '',
  excerpt: '',
  featuredImageUrl: null,
  isPublished: false,
  categoryId: null,
  tagIds: [],
};

export function useBlogFormBrain({
  defaultValues = {},
  onSubmit,
}: {
  defaultValues?: Partial<BlogFormValues>;
  onSubmit: (values: BlogFormValues) => void;
}) {
  const autoSlug = useBoolEngine(true);
  const form = useForm<BlogFormValues>({
    resolver: zodResolver(formSchema),
    defaultValues: defaultValues ?? _defaultValues,
  });
  const title = form.watch('title');
  const onFormSubmit = form.handleSubmit(onSubmit);

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

  return {
    form,
    sanitizeSlug,
    autoSlug,
    onSubmit: onFormSubmit,
  };
}
