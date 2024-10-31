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
    title: 'Blog Posts | Your Site Name',
    description:
      'Read our latest blog posts about technology and software development',
    openGraph: {
      title: 'Blog Posts | Your Site Name',
      description:
        'Read our latest blog posts about technology and software development',
    },
  };
}

export default async function BlogPage({ searchParams }: Props) {
  const page = 1;
  // const page = Number(searchParams?.page) || 1;

  const response = await api.get<PostListResponse>('/post/v1/list/published', {
    params: {
      page,
      // per_page: perPage,
    },
  });

  const { data: posts, total, perPage } = response.data;
  const totalPages = Math.ceil(total / perPage);

  return (
    <main className="container mx-auto py-8 px-4">
      <h1 className="text-4xl font-bold mb-8">Blog Posts</h1>

      <div className="space-y-8">
        {posts.map((post) => (
          <article
            key={post.id}
            className="p-6 bg-white dark:bg-zinc-900 rounded-lg shadow-sm hover:shadow-md transition-shadow"
          >
            <header className="mb-4">
              <h2 className="text-2xl font-semibold mb-2">{post.title}</h2>
              <div className="flex flex-wrap gap-3 text-sm text-gray-600 dark:text-gray-400">
                <span>By {post.author.name}</span>
                <span>•</span>
                <span>
                  {Math.ceil(post.content.split(' ').length / 200)} min read
                </span>
                <span>•</span>
                <time dateTime={post.publishedAt || ''}>
                  {post.publishedAt
                    ? new Date(post.publishedAt).toLocaleDateString()
                    : 'Draft'}
                </time>
              </div>
            </header>

            <p className="text-gray-600 dark:text-gray-300 mb-4">
              {post.excerpt}
            </p>

            <footer className="flex flex-wrap gap-2">
              {post.category && (
                <span className="px-3 py-1 bg-blue-100 dark:bg-blue-900 text-blue-800 dark:text-blue-100 rounded-full text-sm">
                  {post.category.name}
                </span>
              )}
              {post.tags.map((tag) => (
                <span
                  key={tag.id}
                  className="px-3 py-1 bg-gray-100 dark:bg-zinc-800 text-gray-700 dark:text-gray-300 rounded-full text-sm"
                >
                  {tag.name}
                </span>
              ))}
            </footer>
          </article>
        ))}
      </div>

      {totalPages > 1 && (
        <nav className="mt-8 flex justify-center gap-4" aria-label="Pagination">
          {page > 1 && (
            <a
              href={`?page=${page - 1}`}
              className="px-4 py-2 bg-white dark:bg-zinc-900 rounded-lg hover:bg-gray-50 dark:hover:bg-zinc-800 transition-colors"
              rel="prev"
            >
              Previous
            </a>
          )}
          {page < totalPages && (
            <a
              href={`?page=${page + 1}`}
              className="px-4 py-2 bg-white dark:bg-zinc-900 rounded-lg hover:bg-gray-50 dark:hover:bg-zinc-800 transition-colors"
              rel="next"
            >
              Next
            </a>
          )}
        </nav>
      )}
    </main>
  );
}
