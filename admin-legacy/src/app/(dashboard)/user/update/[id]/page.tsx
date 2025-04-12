'use client';

import { useParams } from 'next/navigation';
import { useUpdateUserBrain } from './brain';
import { UserForm } from '@/components/user-form';
import { ContentLoader } from '@/components/content-loader';
import { ContentError } from '@/components/content-error';

export default function UpdateUserPage() {
  const params = useParams<{ id: string }>();
  const brain = useUpdateUserBrain(Number(params.id));

  if (!brain.loaded) {
    return <ContentLoader text="Loading user..." />;
  }

  if (brain.error) {
    return <ContentError title="Failed to load user" />;
  }

  return (
    <div className="container py-10 m-auto">
      <UserForm
        user={brain.user!}
        title="Update user"
        submitLabel="Update"
        loading={brain.loading}
        onSubmit={brain.onSubmit}
      />
    </div>
  );
}
