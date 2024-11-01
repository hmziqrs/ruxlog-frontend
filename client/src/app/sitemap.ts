import { api } from '@/services/api';
import { PostSiteMap } from '@/types';
import type { MetadataRoute } from 'next';

export const revalidate = 60 * 60 * 24; // 1 hour

export default async function sitemap(): Promise<MetadataRoute.Sitemap> {
  const posts = await api.post<PostSiteMap[]>(`/post/v1/sitemap`);

  const sitemapItems: MetadataRoute.Sitemap = [
    {
      url: '/',
      lastModified: new Date('2024-11-01'),
      changeFrequency: 'daily',
      priority: 1,
    },
    {
      url: '/about',
      lastModified: new Date('2024-11-01'),
      changeFrequency: 'weekly',
      priority: 0.8,
    },
    {
      url: '/contact',
      lastModified: new Date('2024-11-01'),
      changeFrequency: 'weekly',
      priority: 0.8,
    },
    {
      url: '/changelog',
      lastModified: new Date('2024-11-01'),
      changeFrequency: 'weekly',
      priority: 0.8,
    },
    {
      url: '/privacy-policy',
      lastModified: new Date('2024-11-01'),
      changeFrequency: 'weekly',
      priority: 0.8,
    },
    {
      url: '/terms-of-service',
      lastModified: new Date('2024-11-01'),
      changeFrequency: 'weekly',
      priority: 0.8,
    },
    ...posts.map((post) => ({
      url: `/post/${post.slug}`,
      lastModified: new Date(post.updatedAt),
      changeFrequency: 'weekly' as const,
      priority: 0.8,
    })),
  ];

  return sitemapItems;
}
