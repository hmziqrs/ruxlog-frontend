'use client';

import { useParams } from 'next/navigation';
import { useUpdatePostBrain } from './brain';
import { BlogForm } from '@/components/blog-form/blog-form';
import { ContentLoader } from '@/components/content-loader';
import { ContentError } from '@/components/content-error';

export default function UpdatePostPage() {
  const params = useParams<{ id: string }>();
  const brain = useUpdatePostBrain(Number(params.id));

  if (!brain.loaded) {
    return <ContentLoader text="Loading post..." />;
  }

  if (brain.error) {
    return <ContentError title="Failed to load post" />;
  }

  return (
    <div className="container py-10 m-auto">
      <BlogForm
        post={brain.post!}
        title="Update post"
        submitLabel="update"
        loading={brain.loading}
        onSubmit={brain.onSubmit}
      />
    </div>
  );
}
