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
    authors: [{ name: post.author.name, url: `/author/${post.author.id}` }],
    keywords: post.tags.map((tag) => tag.name).join(', '),
    category: post.category?.name,
    openGraph: {
      title: post.title,
      description: post.excerpt,
      type: 'article',
      publishedTime: post.publishedAt || post.createdAt,
      modifiedTime: post.updatedAt,
      authors: [post.author.name],
      images: post.featuredImageUrl ? [post.featuredImageUrl] : [],
    },
    twitter: {
      card: 'summary_large_image',
      title: post.title,
      description: post.excerpt,
      images: post.featuredImageUrl ? [post.featuredImageUrl] : [],
    },
  };
}

export default async function PostPage({ params }: PostProps) {
  const post = await api.post<Post>(`/post/v1/view/${params.slug}`, null, {
    next: { revalidate: 60 * 60 * 24 },
    cache: 'default',
  });

  const jsonLd = {
    '@context': 'https://schema.org',
    '@type': 'BlogPosting',
    headline: post.title,
    description: post.excerpt,
    image: post.featuredImageUrl,
    datePublished: post.publishedAt || post.createdAt,
    dateModified: post.updatedAt,
    author: {
      '@type': 'Person',
      name: post.author.name,
      image: post.author.avatar,
    },
  };

  return (
    <>
      <script
        type="application/ld+json"
        dangerouslySetInnerHTML={{ __html: JSON.stringify(jsonLd) }}
      />
      <article className="container mx-auto px-4 py-8">
        {post.featuredImageUrl && (
          <img
            src={post.featuredImageUrl}
            alt={post.title}
            className="w-full h-64 object-cover rounded-lg mb-8"
          />
        )}

        <header className="mb-8">
          <h1 className="text-3xl font-bold mb-2 dark:text-white">
            {post.title}
          </h1>

          <div className="flex items-center gap-4 text-sm text-gray-600 dark:text-gray-400 mb-4">
            <div className="flex items-center">
              {post.author.avatar && (
                <img
                  src={post.author.avatar}
                  alt={post.author.name}
                  className="w-6 h-6 rounded-full mr-2"
                />
              )}
              <span>{post.author.name}</span>
            </div>
            <time dateTime={post.createdAt}>
              {new Date(post.createdAt).toLocaleDateString()}
            </time>
            <div>👁 {post.viewCount} views</div>
            <div>❤️ {post.likesCount} likes</div>
          </div>

          {post.category && (
            <div className="mb-2">
              <span className="bg-gray-100 dark:bg-gray-800 px-3 py-1 rounded-full text-sm">
                {post.category.name}
              </span>
            </div>
          )}

          {post.tags.length > 0 && (
            <div className="flex gap-2 flex-wrap">
              {post.tags.map((tag) => (
                <span
                  key={tag.id}
                  className="bg-gray-100 dark:bg-gray-800 px-2 py-1 rounded-full text-xs"
                >
                  #{tag.name}
                </span>
              ))}
            </div>
          )}
        </header>

        <div className="prose dark:prose-invert prose-sm max-w-none">
          <ReactMarkdown>{post.content}</ReactMarkdown>
        </div>
      </article>
    </>
  );
}
