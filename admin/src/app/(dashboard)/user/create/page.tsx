'use client';
import { useNewUserBrain } from './brain';
import { UserForm } from '@/components/user-form';

export default function NewUserPage() {
  const brain = useNewUserBrain();

  return (
    <div className="container py-10 m-auto">
      <UserForm
        title="Create user"
        submitLabel="Create"
        loading={brain.loading}
        onSubmit={brain.onSubmit}
      />
    </div>
  );
}
