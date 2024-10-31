import Link from 'next/link';

export default function NotFound() {
  return (
    <div className="container mx-auto py-16 px-4">
      <div className="max-w-xl mx-auto text-center">
        <h2 className="text-2xl font-bold mb-4 text-zinc-900 dark:text-zinc-100">
          Page Not Found
        </h2>
        <p className="text-zinc-600 dark:text-zinc-400 mb-6">
          The requested blog page could not be found.
        </p>
        <Link
          href="/"
          className="px-4 py-2 bg-zinc-800 hover:bg-zinc-600 dark:bg-zinc-700 dark:hover:bg-zinc-600 text-white rounded-lg transition-colors duration-300"
        >
          Return Home
        </Link>
      </div>
    </div>
  );
}
