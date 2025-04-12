'use client';
import { useNewTagBrain } from './brain';
import { TagForm } from '@/components/tag-form';

export default function NewTagPage() {
  const brain = useNewTagBrain();

  return (
    <div className="container py-10 m-auto">
      <TagForm
        title="Create tag"
        submitLabel="Create"
        loading={brain.loading}
        onSubmit={brain.onSubmit}
      />
    </div>
  );
}
