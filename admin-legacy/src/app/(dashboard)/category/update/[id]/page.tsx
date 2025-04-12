'use client';

import { useParams } from 'next/navigation';
import { useUpdateCategoryBrain } from './brain';
import { CategoryForm } from '@/components/category-form';
import { ContentLoader } from '@/components/content-loader';
import { ContentError } from '@/components/content-error';

export default function UpdateCategoryPage() {
  const params = useParams<{ id: string }>();
  const brain = useUpdateCategoryBrain(Number(params.id));

  if (!brain.loaded) {
    return <ContentLoader text="Loading category..." />;
  }

  if (brain.error) {
    return <ContentError title="Failed to load category" />;
  }

  return (
    <div className="container py-10 m-auto">
      <CategoryForm
        category={brain.category!}
        title="Update category"
        submitLabel="Update"
        loading={brain.loading}
        onSubmit={brain.onSubmit}
      />
    </div>
  );
}
