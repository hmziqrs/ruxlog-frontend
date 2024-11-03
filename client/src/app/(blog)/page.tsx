import { notFound } from 'next/navigation';
import Link from 'next/link';
import { api } from '@/services/api';
import { Post } from '@/types';
import { Metadata } from 'next';
import { Folder, User2, Calendar, Clock, Heart, Eye } from 'lucide-react';
import { MetaPill } from '@/components/MetaPill';

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
    const page = Math.max(1, Number(searchParams?.page) || 1);

    const response = await api.post<PostListResponse>(
      '/post/v1/list/published',
      { page }
    );

    const { data: posts, total, perPage } = response;

    if (!posts?.length && page !== 1) {
      notFound();
    }

    // const totalPages = 12;
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
        <div className="container mx-auto px-4 sm:px-5 py-6 sm:py-12">
          <div className="grid gap-4 sm:gap-8 md:grid-cols-2">
            {posts.map((post) => (
              <Link
                key={post.id}
                href={`/post/${post.slug}`}
                className="group block"
              >
                <article className="h-full bg-white dark:bg-zinc-900 rounded-lg sm:rounded-xl shadow-sm hover:shadow-xl transition-all duration-300 overflow-hidden">
                  <div className="p-4 sm:p-6 flex flex-col h-full">
                    <h2 className="text-xl font-semibold group-hover:text-zinc-600 dark:group-hover:text-zinc-300 transition-colors line-clamp-2">
                      {post.title}
                    </h2>
                    <div className="h-2" />
                    <p className="font-mono text-zinc-600 dark:text-zinc-400 text-sm mb-4 line-clamp-2">
                      {post.excerpt}
                    </p>

                    <div className="mt-auto">
                      <div className="flex flex-wrap items-center gap-2.5 sm:gap-4">
                        {post.category && (
                          <MetaPill icon={Folder} label={post.category.name} />
                        )}
                        <MetaPill icon={User2} label={post.author.name} />
                        <MetaPill
                          icon={Clock}
                          label={Math.ceil(post.content.split(' ').length / 80)}
                          suffix="min"
                        />
                        {post.publishedAt && (
                          <MetaPill
                            icon={Calendar}
                            label={new Intl.DateTimeFormat('en-US', {
                              month: 'short',
                              day: 'numeric',
                              year: 'numeric',
                            }).format(new Date(post.publishedAt))}
                          />
                        )}
                        <MetaPill
                          icon={Heart}
                          label={post.likesCount || 0}
                          suffix="likes"
                        />
                        <MetaPill
                          icon={Eye}
                          label={post.viewCount || 0}
                          suffix="views"
                        />
                      </div>
                    </div>
                  </div>
                </article>
              </Link>
            ))}
          </div>
          {totalPages > 1 && (
            <nav
              className="flex justify-center sm:gap-3 gap-2 mt-6 sm:text-base text-xs"
              aria-label="Pagination"
            >
              {getPageNumbers(page, totalPages).map((pageNum, idx) =>
                pageNum === '...' ? (
                  <span
                    key={`ellipsis-${idx}`}
                    className="sm:px-3 px-1 py-1 rounded"
                  >
                    ...
                  </span>
                ) : (
                  <Link
                    key={`page-${pageNum}`}
                    href={`?page=${pageNum}`}
                    className={`transition-colors rounded w-8 h-8 sm:w-10 sm:h-10
                      flex items-center justify-center
                       ${
                         pageNum === page
                           ? 'bg-zinc-900 text-white dark:bg-white dark:text-zinc-900'
                           : 'bg-white dark:bg-zinc-900 hover:bg-zinc-50 dark:hover:bg-zinc-800 border border-zinc-200 dark:border-zinc-800'
                       }`}
                  >
                    <span>{pageNum}</span>
                  </Link>
                )
              )}
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
