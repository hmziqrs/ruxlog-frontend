'use client';

import { useEffect } from 'react';
import { analytics } from '@/services/analytics';
import { logEvent } from 'firebase/analytics';
import { usePathname, useSearchParams } from 'next/navigation';

export function useTrackScreenViews() {
  const pathname = usePathname();
  const searchParams = useSearchParams();

  useEffect(() => {
    if (!pathname || !analytics) return;

    const fullPath = searchParams.toString()
      ? `${pathname}?${searchParams.toString()}`
      : pathname;

    logEvent(analytics!, 'screen_view', {
      firebase_screen: fullPath,
      firebase_screen_class: pathname,
      page_title: pathname,
      page_location: fullPath,
      page_path: fullPath,
    });
  }, [pathname, searchParams]); // Dependencies include both pathname and searchParams
}
