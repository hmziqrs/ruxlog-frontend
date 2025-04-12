import { usePathname } from 'next/navigation';
import { useMemo } from 'react';

// if (pathname === '/') {
//   return [
//     {
//       href: '/',
//       label: 'Home',
//       isLast: true,
//     },
//   ];
// }
export function useDashboardLayout() {
  const pathname = usePathname();

  const breadcrumbs = useMemo(() => {
    const segments = pathname
      .split('/')
      .filter(Boolean)
      .map((segment) => segment.toLowerCase());

    return segments.map((segment, index) => {
      const href = `/${segments.slice(0, index + 1).join('/')}`;
      const label = segment.charAt(0).toUpperCase() + segment.slice(1);
      return {
        label,
        href,
        isLast: index === segments.length - 1,
      };
    });
  }, [pathname]);

  return {
    breadcrumbs,
  };
}
