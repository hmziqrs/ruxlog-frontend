'use client';

import { useDidMount, usePrev } from '@/hooks/react-hooks';
import { useAuth } from '@/store/auth';
import { useRouter, usePathname } from 'next/navigation';
import { useEffect } from 'react';
import { toast } from 'sonner';
import Image from 'next/image';

const unAuthRoutes = ['/auth'];

export default function AuthGaurd({ children }: { children: React.ReactNode }) {
  const auth = useAuth();
  const router = useRouter();
  const pathname = usePathname();
  const prevUser = usePrev(auth.data.user);
  const initPrevState = usePrev(auth.state.init);
  const didMount = useDidMount();

  const blockAction = [
    auth.state.init.loading,
    auth.state.login.loading,
    !didMount,
    !auth.state.init.init,
  ].some((x) => x);

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
    if (blockAction) return;
    if (unAuthRoutes.includes(pathname) && auth.data.user) {
      router.push('/');
    }
    if (!unAuthRoutes.includes(pathname) && !auth.data.user) {
      router.push('/auth');
    }
  }, [auth.data.user, blockAction, prevUser, router]);

  if (blockAction) {
    return (
      <div
        className={
          'fixed h-full w-full bg-white flex justify-center items-center flex-col bg-opacity-15'
        }
      >
        <div className="relative w-[200px] h-[200px] md:w-[280px] md:h-[280px]">
          <Image
            src={'/logo.png'}
            alt="Loading..."
            fill
            className="object-contain"
            priority
          />
        </div>
        <div className="h-10" />
        <div className="animate-spin rounded-full h-16 w-16 border-t-2 border-b-2 dark:border-gray-100 border-gray-900"></div>
      </div>
    );
  }

  return children;
}
