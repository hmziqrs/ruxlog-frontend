import { Metadata } from 'next';
import ReactMarkdown from 'react-markdown';
import { api } from '@/services/api';
import { Post } from '@/types';

interface PostProps {
  params: {
    slug: string;
  };
}

export async function generateMetadata({
  params,
}: PostProps): Promise<Metadata> {
  const post = await api.post<Post>(`/post/v1/view/${params.slug}`);
  return {
    title: post.title,
    description: post.excerpt || post.title,
  };
}

export default async function PostPage({ params }: PostProps) {
  const post = await api.post<Post>(`/post/v1/view/${params.slug}`, null, {
    next: { revalidate: 60 * 60 * 24 },
    cache: 'default',
  });

  return (
    <article className="container md:max-w-6xl mx-auto px-4 py-8">
      <header className="mb-8">
        <h1 className="text-3xl font-bold mb-2 dark:text-white">
          {post.title}
        </h1>
        <time className="text-gray-600 dark:text-gray-400">
          {new Date(post.createdAt).toLocaleDateString()}
        </time>
      </header>

      <div className="prose dark:prose-invert prose-sm max-w-none">
        <ReactMarkdown>{post.content}</ReactMarkdown>
      </div>
    </article>
  );
}
