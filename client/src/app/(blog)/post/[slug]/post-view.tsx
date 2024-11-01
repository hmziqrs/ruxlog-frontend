'use client';

import { useDidMount } from '@/lib/react-hooks';
import { api } from '@/services/api';
import { useEffect } from 'react';

interface Props {
  id: number;
}

export function PostView({ id }: Props) {
  const didMount = useDidMount();
  useEffect(() => {
    if (didMount) return;
    api.post(`/post/v1/track_view/${id}`);
  }, [didMount, id]);
  return <></>;
}
