import { useEffect } from 'react';
import { useUser } from '@/store/user';
import { toast } from 'sonner';
import { usePrev } from '@/hooks/react-hooks';
import { useRouter } from 'next/navigation';

export function useNewUserBrain() {
  const user = useUser();
  const prevAddState = usePrev(user.state.add);
  const router = useRouter();
  const { loading } = user.state.add;

  function onSubmit(data: any) {
    user.actions.add(data);
  }

  useEffect(() => {
    if (prevAddState?.loading && !user.state.add.loading) {
      if (user.state.add.success) {
        router.push('/user');
        user.actions.list();
        toast.success('User created successfully!');
      } else if (user.state.add.error) {
        toast.error(user.state.add.message ?? 'Failed to create user');
      }
    }
  }, [user.state.add, prevAddState]);

  return {
    loading,
    onSubmit,
  };
}
