'use client';

import { useDidMount, usePrev } from '@/hooks/react-hooks';
import { useAuth } from '@/store/auth';
import { useRouter, usePathname } from 'next/navigation';
import { useEffect } from 'react';
import { toast } from 'sonner';

const unAuthRoutes = ['/'];

export default function AuthGaurd({ children }: { children: React.ReactNode }) {
  const auth = useAuth();
  const router = useRouter();
  const pathname = usePathname();
  const prevUser = usePrev(auth.data.user);

  // console.log('auth', auth);
  // console.log('router', router);
  // console.log('pathname', pathname);

  const initPrevState = usePrev(auth.state.init);
  // const loginPrevState = usePrev(auth.state.login);
  const didMount = useDidMount();

  useEffect(() => {
    if (initPrevState?.init || didMount) return;

    auth.actions.init();
  }, [initPrevState, auth.actions, didMount]);

  useEffect(() => {
    if (initPrevState?.loading && !auth.state.init.loading) {
      if (auth.state.init.success) {
        toast.success('Signed In Successfully!');
      } else if (auth.state.init.error) {
        toast.error(auth.state.init?.message ?? 'An error occurred!');
      }
    }
  }, [auth.state.init, initPrevState, auth.actions]);

  useEffect(() => {
    console.log(pathname);
    if (unAuthRoutes.includes(pathname) && auth.data.user) {
      router.push('/dashboard');
    }
  }, [auth.data.user, prevUser, router]);

  return children;
}
