import { Metadata } from 'next';
import ReactMarkdown from 'react-markdown';
import { api } from '@/services/api';
import { Post } from '@/types';
import { PostView } from './post-view';

interface PostProps {
  params: {
    slug: string;
  };
}

export async function generateMetadata({
  params,
}: PostProps): Promise<Metadata> {
  const post = await api.post<Post>(`/post/v1/view/${params.slug}`);
  const publishDate = post.publishedAt || post.createdAt;

  return {
    title: post.title,
    description: post.excerpt || post.title,
    authors: [{ name: post.author.name, url: `/author/${post.author.id}` }],
    keywords: post.tags.map((tag) => tag.name).join(', '),
    category: post.category?.name,
    // Basic SEO
    robots: {
      index: true,
      follow: true,
      googleBot: {
        index: true,
        follow: true,
        'max-video-preview': -1,
        'max-image-preview': 'large',
        'max-snippet': -1,
      },
    },
    alternates: {
      canonical: `${process.env.NEXT_PUBLIC_SITE_URL}/post/${post.slug}`,
    },
    openGraph: {
      title: post.title,
      description: post.excerpt,
      type: 'article',
      publishedTime: publishDate,
      modifiedTime: post.updatedAt,
      authors: [post.author.name],
      images: post.featuredImageUrl
        ? [
            {
              url: post.featuredImageUrl,
              width: 1200,
              height: 630,
              alt: post.title,
            },
          ]
        : [],
      siteName: process.env.NEXT_PUBLIC_SITE_NAME,
      locale: 'en_US',
      url: `${process.env.NEXT_PUBLIC_SITE_URL}/post/${post.slug}`,
    },
    // Twitter
    twitter: {
      card: 'summary_large_image',
      title: post.title,
      description: post.excerpt,
      images: post.featuredImageUrl ? [post.featuredImageUrl] : [],
      creator: `@${process.env.NEXT_PUBLIC_USERNAME}`,
      site: `@${process.env.NEXT_PUBLIC_USERNAME}`,
    },
    // Additional Meta Tags
    other: {
      // Pinterest
      'pinterest-rich-pin': 'article',
      // iMessage and other Apple services
      'apple-mobile-web-app-capable': 'yes',
      'apple-mobile-web-app-title': post.title,
      // Microsoft/Bing
      'msapplication-TileImage': post.featuredImageUrl || '',
      // General purpose
      'article:published_time': publishDate,
      'article:modified_time': post.updatedAt,
      'article:author': post.author.name,
      'article:tag': post.tags.map((tag) => tag.name).join(','),
      'article:section': post.category?.name || '',
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
      url: `${process.env.NEXT_PUBLIC_SITE_URL}/author/${post.author.id}`,
    },
    publisher: {
      '@type': 'Organization',
      name: process.env.NEXT_PUBLIC_SITE_NAME,
      logo: {
        '@type': 'ImageObject',
        url: `${process.env.NEXT_PUBLIC_SITE_URL}/logo.png`,
      },
    },
    mainEntityOfPage: {
      '@type': 'WebPage',
      '@id': `${process.env.NEXT_PUBLIC_SITE_URL}/post/${post.slug}`,
    },
    keywords: post.tags.map((tag) => tag.name).join(','),
    articleSection: post.category?.name,
    wordCount: post.content.split(/\s+/).length,
  };

  return (
    <>
      {/* Add more structured data for different platforms */}
      <script
        type="application/ld+json"
        dangerouslySetInnerHTML={{ __html: JSON.stringify(jsonLd) }}
      />
      {/* Yandex specific meta tag */}
      <meta
        name="yandex-verification"
        content="your-yandex-verification-code"
      />
      {/* More specific meta tags for better indexing */}
      <meta
        name="news_keywords"
        content={post.tags.map((tag) => tag.name).join(',')}
      />
      <meta
        name="copyright"
        content={`© ${new Date().getFullYear()} ${process.env.NEXT_PUBLIC_SITE_NAME}`}
      />
      <PostView id={post.id} />
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
