'use client';
import { useNewPostBrain } from './brain';
import { BlogForm } from '@/components/blog-form/blog-form';

export default function NewPostPage() {
  const brain = useNewPostBrain();

  return (
    <div className="container py-10 m-auto">
      <BlogForm
        title="Create post"
        submitLabel="Create"
        loading={brain.loading}
        onSubmit={brain.onSubmit}
      />
    </div>
  );
}
