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
import { Pencil, Trash, Calendar, Folder } from 'lucide-react';
import { ContentLoader } from '@/components/content-loader';
import { ContentError } from '@/components/content-error';
import { ContentNotFound } from '@/components/content-not-found';
import { useParams } from 'next/navigation';

export default function PreviewPage() {
  const params = useParams<{ id: string }>();
  const brain = usePreviewBrain(Number(params.id));

  if (brain.loading && !brain.category) {
    return <ContentLoader text="Loading category..." />;
  }

  if (brain.error) {
    return (
      <ContentError
        title="Failed to load category"
        desc="There was an error loading the category. Please try again."
        onRetry={() => {}}
        onBackLink="/category"
        onBackLabel="Go back"
      />
    );
  }

  if (!brain.category) {
    return (
      <ContentNotFound
        title="Category not found"
        desc="The category you're looking for doesn't exist or has been removed."
        onBackLabel="Back to categories"
        onBackLink="/category"
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
              <Link href={`/category/update/${params.id}`}>
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
                    this category and remove it from our servers.
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

        {/* Featured Image */}
        {brain.category.coverImage && (
          <div className="mb-8">
            <img
              src={brain.category.coverImage}
              alt={brain.category.name}
              className="w-full h-[400px] object-cover rounded-lg"
            />
          </div>
        )}

        {/* Content */}
        <Card className="p-8">
          <h1 className="text-4xl font-bold mb-6">{brain.category.name}</h1>

          {/* Meta Information */}
          <div className="flex flex-wrap gap-4 mb-8 text-sm text-muted-foreground">
            <div className="flex items-center gap-1">
              <Calendar className="h-4 w-4" />
              {new Date(brain.category.createdAt).toLocaleDateString()}
            </div>
            {brain.category.parentId && (
              <div className="flex items-center gap-1">
                <Folder className="h-4 w-4" />
                <Badge variant="secondary">
                  Parent ID: {brain.category.parentId}
                </Badge>
              </div>
            )}
          </div>

          {/* Description */}
          <div className="prose dark:prose-invert max-w-none">
            {brain.category.description}
          </div>
        </Card>
      </div>
      {brain.loading && <ContentLoader text="Loading category..." absolute />}
    </>
  );
}
