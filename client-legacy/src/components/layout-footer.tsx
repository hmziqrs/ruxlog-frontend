import Link from 'next/link';

const links = [
  {
    href: '/privacy-policy',
    label: 'Privacy Policy',
  },
  {
    href: '/terms-of-service',
    label: 'Terms of Service',
  },
  {
    href: '/rss',
    label: 'RSS',
  },
  {
    href: '/sitemap.xml',
    label: 'Sitemap',
  },
];

export function LayoutFooter() {
  return (
    <footer className="dark:bg-zinc-900/50 bg-zinc-100/50 p-6">
      <div className="container mx-auto text-center">
        <div className="font-mono text-sm">
          &copy; {new Date().getFullYear()}, Built with Tailwind CSS, Next.js
          and ❤️ by hmziqrs
        </div>
        <div className="h-2" />
        <Link href="/changelog">
          <p className="text-sm dark:text-zinc-200 text-zinc-800">
            version 1.0.0 - ( <span className="underline">changelog</span> )
          </p>
        </Link>
        <div className="h-2" />
        <div className="space-x-4 text-xs dark:text-zinc-700">
          {links.map(({ label, href }) => (
            <Link
              key={label}
              href={href}
              title={label}
              aria-label={label}
              className="hover:underline"
            >
              {label}
            </Link>
          ))}
        </div>
      </div>
    </footer>
  );
}
