'use client';
import { useNewCategoryBrain } from './brain';
import { CategoryForm } from '@/components/category-form';

export default function NewCategoryPage() {
  const brain = useNewCategoryBrain();

  return (
    <div className="container py-10 m-auto">
      <CategoryForm
        title="Create category"
        submitLabel="Create"
        loading={brain.loading}
        onSubmit={brain.onSubmit}
      />
    </div>
  );
}
