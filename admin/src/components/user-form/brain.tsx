import * as z from 'zod';
import { zodResolver } from '@hookform/resolvers/zod';
import { useForm } from 'react-hook-form';
import { useEffect } from 'react';

const formSchema = z.object({
  name: z.string().min(1, 'Name is required'),
  email: z.string().email('Invalid email address'),
  role: z.enum(['super-admin', 'admin', 'moderator', 'author', 'user']),
  avatar: z.string().url('Invalid URL').nullable(),
  isVerified: z.boolean(),
});

export type UserFormValues = z.infer<typeof formSchema>;

const _defaultValues: UserFormValues = {
  name: '',
  email: '',
  role: 'user',
  avatar: null,
  isVerified: false,
};

export function useUserFormBrain({
  defaultValues = {},
  onSubmit,
}: {
  defaultValues?: Partial<UserFormValues>;
  onSubmit: (values: UserFormValues) => void;
}) {
  const form = useForm<UserFormValues>({
    resolver: zodResolver(formSchema),
    defaultValues: { ..._defaultValues, ...defaultValues },
  });

  const onFormSubmit = form.handleSubmit(onSubmit);

  return {
    form,
    onFormSubmit,
  };
}
