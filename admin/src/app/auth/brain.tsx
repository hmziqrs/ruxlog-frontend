import * as z from 'zod';
import { zodResolver } from '@hookform/resolvers/zod';
import { useForm } from 'react-hook-form';
import { useAuth } from '@/store/auth';
import { useEffect } from 'react';
import { toast } from 'sonner';
import { usePrev } from '@/hooks/react-hooks';

const formSchema = z.object({
  email: z.string().email({
    message: 'Please enter a valid email address.',
  }),
  password: z.string().min(4, {
    message: 'Password must be at least 4 characters.',
  }),
});

export function useAuthBrain() {
  const auth = useAuth();
  const form = useForm<z.infer<typeof formSchema>>({
    resolver: zodResolver(formSchema),
    defaultValues: {
      email: 'superadmin@blog.hmziq.rs',
      password: 'hamza@gamil.com',
    },
  });
  const loginPrevState = usePrev(auth.state.login);
  const { loading } = auth.state.login;

  function onSubmit(values: z.infer<typeof formSchema>) {
    console.log(values);
    auth.actions.login(values);
    // Add your login logic here
  }

  useEffect(() => {
    if (loginPrevState?.loading && !auth.state.login.loading) {
      if (auth.state.login.success) {
        toast.success('Signed In Successfully!');
      } else if (auth.state.login.error) {
        toast.error(auth.state.login?.message ?? 'An error occurred!');
      }
    }
  }, [auth.state.login, loginPrevState, auth.actions]);

  return {
    loading,
    form,
    onSubmit,
  };
}
