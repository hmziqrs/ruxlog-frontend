'use client';
import { usePreviewBrain } from './brain';
import { Button } from '@/components/ui/button';
import { Switch } from '@/components/ui/switch';
import { Badge } from '@/components/ui/badge';
import { Card } from '@/components/ui/card';
import { Markdown } from '@/components/markdown';
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
import {
  Pencil,
  Trash,
  Clock,
  Calendar,
  User,
  Tag,
  Folder,
  Book,
} from 'lucide-react';
import { ContentLoader } from '@/components/content-loader';
import { ContentError } from '@/components/content-error';
import { ContentNotFound } from '@/components/content-not-found';
import { useParams } from 'next/navigation';

export default function PreviewPage() {
  const params = useParams<{ id: string }>();
  const brain = usePreviewBrain(Number(params.id));

  if (brain.loading && !brain.post) {
    return <ContentLoader text="Loading post..." />;
  }

  if (brain.error) {
    return (
      <ContentError
        title="Failed to load post"
        desc="There was an error loading the post. Please try again."
        onRetry={() => {}}
        onBackLink="/post"
        onBackLabel="Go back"
      />
    );
  }

  if (!brain.post) {
    return (
      <ContentNotFound
        title="Post not found"
        desc="The post you're looking for doesn't exist or has been removed."
        onBackLabel="Back to posts"
        onBackLink="/post"
      />
    );
  }

  const calculateReadingTime = (content: string) => {
    const wordsPerMinute = 200;
    const words = content.split(/\s+/).length;
    const minutes = Math.ceil(words / wordsPerMinute);
    return `${minutes} min read`;
  };

  return (
    <>
      <div className="container mx-auto py-8 px-4">
        {/* Control Bar */}
        <div className="flex justify-between items-center mb-8">
          <div className="flex items-center gap-4">
            <Switch
              checked={brain.post.isPublished}
              onCheckedChange={brain.handleTogglePublish}
            />
            <span className="text-sm text-muted-foreground">
              {brain.post.isPublished ? 'Published' : 'Draft'}
            </span>
          </div>
          <div className="flex gap-2">
            <Button variant="outline" asChild>
              <Link href={`/post/update/${params.id}`}>
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
                    this post and remove it from our servers.
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
        {brain.post.featuredImageUrl && (
          <div className="mb-8">
            <img
              src={brain.post.featuredImageUrl}
              alt={brain.post.title}
              className="w-full h-[400px] object-cover rounded-lg"
            />
          </div>
        )}

        {/* Content */}
        <Card className="p-8">
          <h1 className="text-4xl font-bold mb-6">{brain.post.title}</h1>

          {/* Meta Information */}
          <div className="flex flex-wrap gap-4 mb-8 text-sm text-muted-foreground">
            <div className="flex items-center gap-1">
              <User className="h-4 w-4" />
              {brain.post.author?.name ?? 'Anonymous'}
            </div>
            <div className="flex items-center gap-1">
              <Calendar className="h-4 w-4" />
              {new Date(brain.post.createdAt).toLocaleDateString()}
            </div>
            <div className="flex items-center gap-1">
              <Clock className="h-4 w-4" />
              {new Date(brain.post.updatedAt).toLocaleDateString()}
            </div>
            <div className="flex items-center gap-1">
              <Book className="h-4 w-4" />
              {calculateReadingTime(brain.post.content)}
            </div>
            {brain.post.category && (
              <div className="flex items-center gap-1">
                <Folder className="h-4 w-4" />
                <Badge variant="secondary">{brain.post.category.name}</Badge>
              </div>
            )}
            {brain.post?.tags?.length > 0 && (
              <div className="flex items-center gap-1">
                <Tag className="h-4 w-4" />
                <div className="flex gap-1">
                  {brain.post.tags.map((tag) => (
                    <Badge key={tag.id} variant="outline">
                      {tag.name}
                    </Badge>
                  ))}
                </div>
              </div>
            )}
          </div>

          {/* Markdown Content */}
          <div className="prose dark:prose-invert max-w-none">
            <Markdown>{brain.post.content}</Markdown>
          </div>
        </Card>
      </div>
      {brain.loading && <ContentLoader text="Loading post..." absolute />}
    </>
  );
}
