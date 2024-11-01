// import Link from 'next/link';
import { Metadata } from 'next';
import { cn } from '@/lib/utils';

interface ChangelogEntry {
  date: Date;
  version: string;
  changes: {
    type: 'new' | 'improved' | 'fixed' | 'removed';
    description: string;
  }[];
}

export const metadata: Metadata = {
  title: 'Changelog | Your Site Name',
  description: 'Track all updates and improvements to our platform',
};

const getChangeTypeStyles = (type: ChangelogEntry['changes'][0]['type']) => {
  const styles = {
    new: 'bg-emerald-50 dark:bg-emerald-950 text-emerald-700 dark:text-emerald-300 border-emerald-200 dark:border-emerald-800',
    improved:
      'bg-sky-50 dark:bg-sky-950 text-sky-700 dark:text-sky-300 border-sky-200 dark:border-sky-800',
    fixed:
      'bg-amber-50 dark:bg-amber-950 text-amber-700 dark:text-amber-300 border-amber-200 dark:border-amber-800',
    removed:
      'bg-rose-50 dark:bg-rose-950 text-rose-700 dark:text-rose-300 border-rose-200 dark:border-rose-800',
  };
  return styles[type];
};

const changelog: ChangelogEntry[] = [
  {
    date: new Date('2024-01-15'),
    version: '1.1.0',
    changes: [
      {
        type: 'new',
        description: 'Added markdown support in blog posts',
      },
      {
        type: 'improved',
        description: 'Enhanced code syntax highlighting',
      },
    ],
  },
  {
    date: new Date('2023-12-25'),
    version: '1.0.0',
    changes: [
      {
        type: 'new',
        description: 'Initial release with Next.js 14 and App Router',
      },
      {
        type: 'improved',
        description: 'Blog system with Rust backend integration',
      },
      {
        type: 'fixed',
        description: 'Dark mode implementation with Tailwind CSS',
      },
    ],
  },
  {
    date: new Date('2023-12-01'),
    version: '0.9.0-beta',
    changes: [
      {
        type: 'new',
        description: 'Beta release of the blog platform',
      },
      {
        type: 'new',
        description: 'Added basic authentication system',
      },
    ],
  },
];

export default function ChangelogPage() {
  return (
    <main className="container mx-auto py-8 px-5">
      <h1 className="font-mono text-3xl font-semibold mb-8">Changelog</h1>

      <div className="space-y-6">
        {changelog.map((entry) => (
          <details
            key={entry.version}
            className="group"
            open={entry.version === changelog[0].version} // Latest version open by default
          >
            <summary className="flex items-center gap-3 cursor-pointer list-none border-l-2 border-zinc-200 dark:border-zinc-800 pl-6 py-2 hover:bg-zinc-50 dark:hover:bg-zinc-900">
              <span className="font-mono text-xl">
                {entry.date.toLocaleDateString('en-US', {
                  year: 'numeric',
                  month: 'short',
                  day: 'numeric',
                })}
              </span>
              <span className="px-3 py-1 bg-zinc-100 dark:bg-zinc-800 rounded-full text-sm">
                v{entry.version}
              </span>
              <span className="ml-auto text-zinc-400 group-open:rotate-180 transition-transform">
                ▼
              </span>
            </summary>

            <div className="mt-4 space-y-4 border-l-2 border-zinc-200 dark:border-zinc-800 pl-6">
              {entry.changes.map((change, index) => (
                <div key={index} className="flex items-start gap-3">
                  <span
                    className={cn(
                      getChangeTypeStyles(change.type),
                      'px-3 py-1 rounded-full text-sm border'
                    )}
                  >
                    {change.type}
                  </span>
                  <p className="text-zinc-600 dark:text-zinc-400 pt-1">
                    {change.description}
                  </p>
                </div>
              ))}
            </div>
          </details>
        ))}
      </div>
    </main>
  );
}
