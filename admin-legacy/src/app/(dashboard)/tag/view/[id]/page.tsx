'use client';
import { usePreviewBrain } from './brain';
import { Button } from '@/components/ui/button';
import { Badge } from '@/components/ui/badge';
import { Card } from '@/components/ui/card';
import Link from 'next/link';
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
import { Pencil, Trash, Calendar } from 'lucide-react';
import { ContentLoader } from '@/components/content-loader';
import { ContentError } from '@/components/content-error';
import { ContentNotFound } from '@/components/content-not-found';
import { useParams } from 'next/navigation';

export default function PreviewPage() {
  const params = useParams<{ id: string }>();
  const brain = usePreviewBrain(Number(params.id));

  if (brain.loading && !brain.tag) {
    return <ContentLoader text="Loading tag..." />;
  }

  if (brain.error) {
    return (
      <ContentError
        title="Failed to load tag"
        desc="There was an error loading the tag. Please try again."
        onRetry={() => {}}
        onBackLink="/tag"
        onBackLabel="Go back"
      />
    );
  }

  if (!brain.tag) {
    return (
      <ContentNotFound
        title="Tag not found"
        desc="The tag you're looking for doesn't exist or has been removed."
        onBackLabel="Back to tags"
        onBackLink="/tag"
      />
    );
  }

  return (
    <>
      <div className="container mx-auto py-8 px-4">
        {/* Control Bar */}
        <div className="flex justify-between items-center mb-8">
          <div className="flex gap-2">
            <Button variant="outline" asChild>
              <Link href={`/tag/update/${params.id}`}>
                <Pencil className="h-4 w-4 mr-2" />
                Edit
              </Link>
            </Button>
            <AlertDialog>
              <AlertDialogTrigger asChild>
                <Button variant="destructive">
                  <Trash className="h-4 w-4 mr-2" />
                  Delete
                </Button>
              </AlertDialogTrigger>
              <AlertDialogContent>
                <AlertDialogHeader>
                  <AlertDialogTitle>Are you absolutely sure?</AlertDialogTitle>
                  <AlertDialogDescription>
                    This action cannot be undone. This will permanently delete
                    this tag and remove it from our servers.
                  </AlertDialogDescription>
                </AlertDialogHeader>
                <AlertDialogFooter>
                  <AlertDialogCancel>Cancel</AlertDialogCancel>
                  <AlertDialogAction className="bg-destructive text-destructive-foreground hover:bg-destructive/90">
                    Delete
                  </AlertDialogAction>
                </AlertDialogFooter>
              </AlertDialogContent>
            </AlertDialog>
          </div>
        </div>

        {/* Content */}
        <Card className="p-8">
          <h1 className="text-4xl font-bold mb-6">{brain.tag.name}</h1>

          {/* Meta Information */}
          <div className="flex flex-wrap gap-4 mb-8 text-sm text-muted-foreground">
            <div className="flex items-center gap-1">
              <Calendar className="h-4 w-4" />
              {new Date(brain.tag.createdAt).toLocaleDateString()}
            </div>
          </div>

          {/* Description */}
          <div className="prose dark:prose-invert max-w-none">
            {brain.tag.description}
          </div>
        </Card>
      </div>
      {brain.loading && <ContentLoader text="Loading tag..." absolute />}
    </>
  );
}
