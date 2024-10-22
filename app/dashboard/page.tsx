'use client';
import { useAuth } from '@/store/auth';
import { redirect } from 'next/navigation';

export default function Dashboard() {
  const auth = useAuth();

  if (!auth.data?.user) {
    return redirect('/');
  } else {
    redirect('/dashboard/overview');
  }
}
