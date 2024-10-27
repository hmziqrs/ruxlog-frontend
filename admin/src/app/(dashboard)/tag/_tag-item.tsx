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
import { Tag } from '@/store/tag/types';
import { TagBrain, useTagItemBrain } from './brain';
import Link from 'next/link';
import { Pencil, Trash, Eye } from 'lucide-react';
import { ContentLoader } from '@/components/content-loader';

export const TagItem = ({ tag }: { tag: Tag; brain: TagBrain }) => {
  const tagItemBrain = useTagItemBrain(tag.id);

  return (
    <tr className="hover:bg-zinc-800/50 transition-colors">
      <td className="px-6 py-4 whitespace-nowrap">
        <div className="text-sm font-medium text-zinc-100">{tag.name}</div>
      </td>
      <td className="px-6 py-4 whitespace-nowrap text-sm text-zinc-400">
        {tag.slug}
      </td>
      <td className="px-6 py-4 whitespace-nowrap text-sm text-zinc-400">
        {tag.description}
      </td>
      <td className="px-6 py-4 whitespace-nowrap text-sm text-zinc-400">
        {new Date(tag.createdAt).toLocaleDateString()}
      </td>
      <td className="px-6 py-4 whitespace-nowrap text-sm text-zinc-400">
        {new Date(tag.updatedAt).toLocaleDateString()}
      </td>
      <td className="px-6 py-4 whitespace-nowrap text-sm text-zinc-400">
        <div className="flex items-center gap-2">
          <Link href={`/tag/update/${tag.id}`}>
            <Button size="sm" variant="outline">
              <Pencil className="h-4 w-4" />
            </Button>
          </Link>

          <Button size="sm" variant="outline" asChild>
            <Link href={`/tag/view/${tag.id}`}>
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
                  &quot;{tag.name}&quot; and remove it from our servers.
                </AlertDialogDescription>
              </AlertDialogHeader>
              <AlertDialogFooter>
                <AlertDialogCancel>Cancel</AlertDialogCancel>
                <AlertDialogAction
                  className="bg-destructive text-destructive-foreground hover:bg-destructive/90"
                  onClick={() => tagItemBrain.removeTag()}
                >
                  Delete
                </AlertDialogAction>
              </AlertDialogFooter>
            </AlertDialogContent>
          </AlertDialog>
        </div>
      </td>
      {tagItemBrain.loading && <ContentLoader absolute size="sm" />}
    </tr>
  );
};
