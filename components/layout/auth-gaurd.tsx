'use client';

import { useAuth } from '@/store/auth';
import { useRouter, usePathname } from 'next/navigation';

export default function AuthGaurd({ children }: { children: React.ReactNode }) {
  const auth = useAuth();
  const router = useRouter();
  const pathname = usePathname();

  // console.log('auth', auth);
  // console.log('router', router);
  // console.log('pathname', pathname);

  // const auth = useAuth();
  // const initPrevState = usePrev(auth.state.init);
  // const loginPrevState = usePrev(auth.state.login);
  // const didMount = useDidMount();

  // useEffect(() => {
  //   if (didMount) return;
  //   auth.actions.init();
  // }, [didMount, auth.actions]);

  // useEffect(() => {
  //   if (initPrevState?.loading && !auth.state.init.loading) {
  //     if (auth.state.init.success) {
  //       toast.success('Signed In Successfully!');
  //       redirect('/dashboard');
  //     } else if (auth.state.init.error) {
  //       toast.error(auth.state.init?.message ?? 'An error occurred!');
  //     }
  //   }
  // }, [auth.state.init, initPrevState, auth.actions]);

  // useEffect(() => {
  //   if (loginPrevState?.loading && !auth.state.login.loading) {
  //     if (auth.state.login.success) {
  //       toast.success('Signed In Successfully!');
  //       redirect('/dashboard');
  //     } else if (auth.state.login.error) {
  //       toast.error(auth.state.login?.message ?? 'An error occurred!');
  //     }
  //   }
  // }, [auth.state.login, loginPrevState, auth.actions]);

  return children;
}
