'use client';

import { useTrackScreenViews } from '@/hooks/useTrackScreenViews';
import { Fragment } from 'react';

export function ScreenTracker() {
  useTrackScreenViews();

  return <Fragment />;
}
