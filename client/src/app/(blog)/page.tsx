import { notFound } from 'next/navigation';
import Link from 'next/link';
import { api } from '@/services/api';
import { Post } from '@/types';
import { Metadata } from 'next';

interface Props {
  searchParams: { page?: string };
}

interface PostListResponse {
  data: Post[];
  total: number;
  perPage: number;
}

export function generateMetadata(): Metadata {
  return {
    title: `Blog Posts | ${process.env.NEXT_PUBLIC_SITE_NAME}`,
    description:
      'Read our latest blog posts about technology and software development',
    openGraph: {
      title: `Blog Posts | ${process.env.NEXT_PUBLIC_SITE_NAME}`,
      description:
        'Read our latest blog posts about technology and software development',
    },
  };
}

export default async function BlogPage({ searchParams }: Props) {
  try {
    const page = Number(searchParams?.page) || 1;

    if (page < 1) {
      throw new Error('Invalid page number');
    }

    const response = await api.post<PostListResponse>(
      '/post/v1/list/published',
      { page }
    );

    const { data: posts, total, perPage } = response;

    if (!posts?.length && page !== 1) {
      notFound();
    }

    const totalPages = Math.ceil(total / perPage);

    if (page > totalPages) {
      notFound();
    }

    const getPageNumbers = (current: number, total: number) => {
      const delta = 2;
      const range = [];
      for (
        let i = Math.max(2, current - delta);
        i <= Math.min(total - 1, current + delta);
        i++
      ) {
        range.push(i);
      }

      if (current - delta > 2) {
        range.unshift('...');
      }
      if (current + delta < total - 1) {
        range.push('...');
      }

      range.unshift(1);
      if (total !== 1) {
        range.push(total);
      }

      return range;
    };

    return (
      <main className="min-h-screen">
        <div className="container mx-auto px-5 py-12">
          <div className="grid gap-8 md:grid-cols-2">
            {posts.map((post) => (
              <Link
                key={post.id}
                href={`/post/${post.slug}`}
                className="group block"
              >
                <article className="h-full bg-white dark:bg-zinc-900 rounded-xl shadow-sm hover:shadow-xl transition-all duration-300 overflow-hidden">
                  <div className="p-6 flex flex-col h-full">
                    {post.category && (
                      <span className="text-sm font-medium text-zinc-600 dark:text-zinc-400 mb-2">
                        {post.category.name}
                      </span>
                    )}
                    <h2 className="text-xl font-bold mb-3 group-hover:text-zinc-600 dark:group-hover:text-zinc-300 transition-colors">
                      {post.title}
                    </h2>
                    <p className="text-zinc-600 dark:text-zinc-400 text-sm mb-4 line-clamp-2">
                      {post.excerpt}
                    </p>

                    <div className="mt-auto">
                      <div className="flex items-center gap-3 text-sm text-zinc-500 dark:text-zinc-400 mb-4">
                        <span className="font-medium">{post.author.name}</span>
                        <span>•</span>
                        <span>
                          {Math.ceil(post.content.split(' ').length / 200)} min
                          read
                        </span>
                        {post.publishedAt && (
                          <>
                            <span>•</span>
                            <time
                              dateTime={post.publishedAt}
                              className="font-mono"
                            >
                              {new Date(post.publishedAt).toLocaleDateString()}
                            </time>
                          </>
                        )}
                      </div>

                      <div className="flex flex-wrap gap-2">
                        {post.tags.map((tag) => (
                          <span
                            key={tag.id}
                            className="px-3 py-1 bg-zinc-100 dark:bg-zinc-800 rounded-full text-xs font-medium"
                          >
                            #{tag.name}
                          </span>
                        ))}
                      </div>
                    </div>
                  </div>
                </article>
              </Link>
            ))}
          </div>

          {totalPages > 1 && (
            <nav
              className="mt-12 flex justify-center gap-2"
              aria-label="Pagination"
            >
              <a
                href="?page=1"
                className={`px-3 py-2 bg-white dark:bg-zinc-900 rounded-lg hover:bg-zinc-50 dark:hover:bg-zinc-800 border border-zinc-200 dark:border-zinc-800 transition-colors ${
                  page === 1 ? 'pointer-events-none opacity-50' : ''
                }`}
                aria-label="First page"
              >
                <svg
                  className="w-5 h-5"
                  fill="none"
                  stroke="currentColor"
                  viewBox="0 0 24 24"
                >
                  <path
                    strokeLinecap="round"
                    strokeLinejoin="round"
                    strokeWidth={2}
                    d="M11 19l-7-7 7-7m8 14l-7-7 7-7"
                  />
                </svg>
              </a>

              {getPageNumbers(page, totalPages).map((pageNum, idx) =>
                pageNum === '...' ? (
                  <span key={`ellipsis-${idx}`} className="px-3 py-2">
                    ...
                  </span>
                ) : (
                  <a
                    key={`page-${pageNum}`}
                    href={`?page=${pageNum}`}
                    className={`px-4 py-2 rounded-lg transition-colors ${
                      pageNum === page
                        ? 'bg-zinc-900 text-white dark:bg-white dark:text-zinc-900'
                        : 'bg-white dark:bg-zinc-900 hover:bg-zinc-50 dark:hover:bg-zinc-800 border border-zinc-200 dark:border-zinc-800'
                    }`}
                  >
                    {pageNum}
                  </a>
                )
              )}

              <a
                href={`?page=${totalPages}`}
                className={`px-3 py-2 bg-white dark:bg-zinc-900 rounded-lg hover:bg-zinc-50 dark:hover:bg-zinc-800 border border-zinc-200 dark:border-zinc-800 transition-colors ${
                  page === totalPages ? 'pointer-events-none opacity-50' : ''
                }`}
                aria-label="Last page"
              >
                <svg
                  className="w-5 h-5"
                  fill="none"
                  stroke="currentColor"
                  viewBox="0 0 24 24"
                >
                  <path
                    strokeLinecap="round"
                    strokeLinejoin="round"
                    strokeWidth={2}
                    d="M13 5l7 7-7 7M5 5l7 7-7 7"
                  />
                </svg>
              </a>
            </nav>
          )}
        </div>
      </main>
    );
  } catch (error) {
    if (error instanceof Error) {
      throw error; // This will be caught by the error boundary
    }
    throw new Error('Failed to load blog posts');
  }
}
