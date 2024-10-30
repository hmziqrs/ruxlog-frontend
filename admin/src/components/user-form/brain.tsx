import * as z from 'zod';
import { zodResolver } from '@hookform/resolvers/zod';
import { useForm } from 'react-hook-form';

const formSchema = (isUpdate: boolean) =>
  z.object({
    name: z.string().min(1, 'Name is required'),
    email: z.string().email('Invalid email address'),
    role: z.enum(['super-admin', 'admin', 'moderator', 'author', 'user']),
    // avatar: z.string().url('Invalid URL').nullable(),
    isVerified: z.boolean(),
    password: isUpdate
      ? z.string().optional()
      : z.string().min(1, 'Password is required'),
  });

export type UserFormValues = z.infer<ReturnType<typeof formSchema>>;

const _defaultValues: UserFormValues = {
  name: '',
  email: '',
  role: 'user',
  // avatar: '',
  isVerified: false,
  // password: '',
};

export function useUserFormBrain({
  defaultValues = {},
  onSubmit,
}: {
  defaultValues?: Partial<UserFormValues>;
  onSubmit: (values: UserFormValues) => void;
}) {
  const isUpdate = !!defaultValues?.email;
  const form = useForm<UserFormValues>({
    resolver: zodResolver(formSchema(isUpdate)),
    defaultValues: { ..._defaultValues, ...defaultValues },
  });

  const onFormSubmit = form.handleSubmit(onSubmit);

  return {
    form,
    onFormSubmit,
  };
}
