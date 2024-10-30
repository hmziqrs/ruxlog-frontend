'use client';
import { Button } from '@/components/ui/button';
import {
  AlertDialog,
  AlertDialogAction,
  AlertDialogCancel,
  AlertDialogContent,
  AlertDialogDescription,
  AlertDialogFooter,
  AlertDialogHeader,
  AlertDialogTitle,
  AlertDialogTrigger,
} from '@/components/ui/alert-dialog';
import { User } from '@/store/user/types';
import { UserBrain, useUserItemBrain } from './brain';
import Link from 'next/link';
import { Pencil, Trash, Eye } from 'lucide-react';
import { ContentLoader } from '@/components/content-loader';

export const UserItem = ({ user }: { user: User; brain: UserBrain }) => {
  const userItemBrain = useUserItemBrain(user.id);

  return (
    <tr className="hover:bg-zinc-800/50 transition-colors">
      <td className="px-6 py-4 whitespace-nowrap">
        <div className="text-sm font-medium text-zinc-100">{user.name}</div>
      </td>
      <td className="px-6 py-4 whitespace-nowrap text-sm text-zinc-400">
        {user.email}
      </td>
      <td className="px-6 py-4 whitespace-nowrap text-sm text-zinc-400">
        {user.role}
      </td>
      <td className="px-6 py-4 whitespace-nowrap text-sm text-zinc-400">
        {user.isVerified ? 'Yes' : 'No'}
      </td>
      <td className="px-6 py-4 whitespace-nowrap text-sm text-zinc-400">
        {new Date(user.createdAt).toLocaleDateString()}
      </td>
      <td className="px-6 py-4 whitespace-nowrap text-sm text-zinc-400">
        {new Date(user.updatedAt).toLocaleDateString()}
      </td>
      <td className="px-6 py-4 whitespace-nowrap text-sm text-zinc-400">
        <div className="flex items-center gap-2">
          <Link href={`/user/update/${user.id}`}>
            <Button size="sm" variant="outline">
              <Pencil className="h-4 w-4" />
            </Button>
          </Link>

          <Button size="sm" variant="outline" asChild>
            <Link href={`/user/view/${user.id}`}>
              <Eye className="h-4 w-4" />
            </Link>
          </Button>

          <AlertDialog>
            <AlertDialogTrigger asChild>
              <Button size="sm" variant="destructive">
                <Trash className="h-4 w-4" />
              </Button>
            </AlertDialogTrigger>
            <AlertDialogContent>
              <AlertDialogHeader>
                <AlertDialogTitle>Are you absolutely sure?</AlertDialogTitle>
                <AlertDialogDescription>
                  This action cannot be undone. This will permanently delete
                  &quot;{user.name}&quot; and remove it from our servers.
                </AlertDialogDescription>
              </AlertDialogHeader>
              <AlertDialogFooter>
                <AlertDialogCancel>Cancel</AlertDialogCancel>
                <AlertDialogAction
                  className="bg-destructive text-destructive-foreground hover:bg-destructive/90"
                  onClick={() => userItemBrain.removeUser()}
                >
                  Delete
                </AlertDialogAction>
              </AlertDialogFooter>
            </AlertDialogContent>
          </AlertDialog>
        </div>
      </td>
      {userItemBrain.loading && <ContentLoader absolute size="sm" />}
    </tr>
  );
};
