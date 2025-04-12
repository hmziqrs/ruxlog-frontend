'use client';

import { useParams } from 'next/navigation';
import { useUpdateTagBrain } from './brain';
import { TagForm } from '@/components/tag-form';
import { ContentLoader } from '@/components/content-loader';
import { ContentError } from '@/components/content-error';

export default function UpdateTagPage() {
  const params = useParams<{ id: string }>();
  const brain = useUpdateTagBrain(Number(params.id));

  if (!brain.loaded) {
    return <ContentLoader text="Loading tag..." />;
  }

  if (brain.error) {
    return <ContentError title="Failed to load tag" />;
  }

  return (
    <div className="container py-10 m-auto">
      <TagForm
        tag={brain.tag!}
        title="Update tag"
        submitLabel="Update"
        loading={brain.loading}
        onSubmit={brain.onSubmit}
      />
    </div>
  );
}
