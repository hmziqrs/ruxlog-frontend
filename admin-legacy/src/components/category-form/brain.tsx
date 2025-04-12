import * as z from 'zod';
import { zodResolver } from '@hookform/resolvers/zod';
import { useForm } from 'react-hook-form';
import { useEffect } from 'react';

const formSchema = z.object({
  name: z.string().min(1, 'Name is required'),
  slug: z
    .string()
    .min(1, 'Slug is required')
    .regex(
      /^[a-z0-9-_]+$/,
      'Slug can only contain lowercase letters, numbers, and hyphens'
    )
    .transform((value) => value.toLowerCase()),
  description: z.string().nullable(),
  coverImage: z.string().nullable(),
  logoImage: z.string().nullable(),
  parentId: z.number().nullable(),
});

export type CategoryFormValues = z.infer<typeof formSchema>;

const _defaultValues: CategoryFormValues = {
  name: '',
  slug: '',
  description: null,
  coverImage: null,
  logoImage: null,
  parentId: null,
};

export function useCategoryFormBrain({
  defaultValues = {},
  onSubmit,
}: {
  defaultValues?: Partial<CategoryFormValues>;
  onSubmit: (values: CategoryFormValues) => void;
}) {
  const form = useForm<CategoryFormValues>({
    resolver: zodResolver(formSchema),
    defaultValues: { ..._defaultValues, ...defaultValues },
  });
  const name = form.watch('name');
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
    if (name) {
      form.setValue('slug', sanitizeSlug(name));
    }
  }, [name, form]);

  return {
    form,
    sanitizeSlug,
    onFormSubmit,
  };
}
