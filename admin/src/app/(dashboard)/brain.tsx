import { usePathname } from 'next/navigation';
import { useMemo } from 'react';

export function useDashboardLayout() {
  const pathname = usePathname();

  const breadcrumbs = useMemo(() => {
    if (pathname === '/') {
      return [
        {
          href: '/',
          label: 'Home',
          isLast: true,
        },
      ];
    }
    // Remove first empty string from split
    const segments = pathname.split('/').filter(Boolean);

    return segments.map((segment, index) => {
      const href = `/${segments.slice(0, index + 1).join('/')}`;
      const label = segment
        // Add space between camelCase
        .replace(/([A-Z])/g, ' $1')
        // Capitalize first letter
        .replace(/^./, (str) => str.toUpperCase())
        // Replace hyphens and underscores with spaces
        .replace(/[-_]/g, ' ');

      return {
        href,
        label,
        isLast: index === segments.length - 1,
      };
    });
  }, [pathname]);

  return { breadcrumbs };
}
