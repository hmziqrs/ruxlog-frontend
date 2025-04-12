'use client';

import { useEffect } from 'react';

interface ErrorProps {
  error: Error;
  reset: () => void;
}

export default function Error({ error, reset }: ErrorProps) {
  useEffect(() => {
    // Optionally log to error reporting service
    console.error('Blog page error:', error);
  }, [error]);

  return (
    <div className="container mx-auto py-16 px-4">
      <div className="max-w-xl mx-auto text-center">
        <h2 className="text-2xl font-bold mb-4">Something went wrong!</h2>
        <p className="text-zinc-600 dark:text-zinc-400 mb-6">
          {error.message || 'Failed to load blog posts'}
        </p>
        <button
          onClick={reset}
          className="px-4 py-2 bg-zinc-300 hover:bg-zinc-200 dark:bg-zinc-800/50 dark:hover:bg-zinc-800 dark:text-white rounded-lg transition-colors duration-300"
        >
          Try again
        </button>
      </div>
    </div>
  );
}
