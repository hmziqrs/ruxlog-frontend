import { Metadata } from 'next';
import ReactMarkdown from 'react-markdown';
import { Prism as SyntaxHighlighter } from 'react-syntax-highlighter';
import { dracula } from 'react-syntax-highlighter/dist/esm/styles/prism';
import { api } from '@/services/api';
import { Post } from '@/types';
import { PostView } from './post-view';
import { Clock, Calendar, Heart, Eye, User, Folder } from 'lucide-react';
import { MetaPill } from '@/components/MetaPill';
import Image from 'next/image';

interface PostProps {
  params: Promise<{
    slug: string;
  }>;
}

export async function generateMetadata({
  params,
}: PostProps): Promise<Metadata> {
  const { slug } = await params;
  const post = await api.post<Post>(`/post/v1/view/${slug}`);
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
  const { slug } = await params;
  const languageMap: Record<string, string> = {
    rs: 'rust',
  };

  const post = await api.post<Post>(`/post/v1/view/${slug}`, null, {
    next: { revalidate: 60 * 60 * 24 },
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

  const readingTime = Math.ceil(post.content.split(/\s+/).length / 80);

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
      <article className="px-4 py-8">
        {post.featuredImageUrl && (
          <div className="relative h-[300px] mb-8 rounded-xl overflow-hidden">
            <Image
              src={post.featuredImageUrl}
              alt={post.title}
              className="absolute inset-0 w-full h-full object-cover"
            />
            <div className="absolute inset-0 bg-gradient-to-t from-black/60 to-transparent" />
          </div>
        )}

        <header>
          <h1 className="text-2xl sm:text-3xl font-semibold mb-6  leading-tight">
            {post.title}
          </h1>

          <div className="flex flex-wrap items-center sm:gap-4 gap-2.5 mb-6">
            <MetaPill
              icon={Folder}
              label={post.category?.name || 'un categorized'}
            />

            <MetaPill icon={User} label={post.author.name} />
            <MetaPill
              icon={Calendar}
              label={new Date(post.createdAt).toLocaleDateString('en-US', {
                year: 'numeric',
                month: 'long',
                day: 'numeric',
              })}
            />
            <MetaPill icon={Clock} label={`${readingTime} min read`} />
            <MetaPill icon={Eye} label={`${post.viewCount} views`} />
            <MetaPill icon={Heart} label={`${post.likesCount} likes`} />
          </div>
        </header>

        <div className="prose dark:prose-invert max-w-full">
          <ReactMarkdown
            components={{
              code({ className, children, ...props }) {
                const match = /language-(\w+)/.exec(className || '');
                const lang = match ? match[1] : '';
                return match ? (
                  <SyntaxHighlighter
                    PreTag="div"
                    language={languageMap[lang] || lang}
                    style={dracula}
                  >
                    {String(children).replace(/\n$/, '')}
                  </SyntaxHighlighter>
                ) : (
                  <code className={className} {...props}>
                    {children}
                  </code>
                );
              },
            }}
          >
            {post.content}
          </ReactMarkdown>
        </div>
        <div className="h-4" />
        <div className="flex flex-wrap gap-3">
          {post.tags.map((tag) => (
            <span
              key={tag.id}
              className="bg-zinc-100 dark:bg-zinc-800 px-3 py-1.5 rounded-full text-sm"
            >
              #{tag.name}
            </span>
          ))}
        </div>
      </article>
    </>
  );
}
