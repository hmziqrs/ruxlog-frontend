'use client';

import { usePreviewBrain } from './brain';
import { Button } from '@/components/ui/button';
import { Switch } from '@/components/ui/switch';
import { Badge } from '@/components/ui/badge';
import { Card } from '@/components/ui/card';
import { Alert, AlertDescription } from '@/components/ui/alert';
import { Markdown } from '@/components/markdown';
import Link from 'next/link';
import Image from 'next/image';
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
  RefreshCw,
  Clock,
  Calendar,
  User,
  Tag,
  Folder,
  Book,
  Loader2,
} from 'lucide-react';

export default function PreviewPage({ params }: { params: { id: string } }) {
  const brain = usePreviewBrain(Number(params.id));

  if (brain.loading) {
    return (
      <div className="flex justify-center items-center min-h-screen flex-col-reverse">
        <Loader2 className="h-14 w-14 animate-spin" />
        <div className="h-4" />
        <Image alt="Logo" src="/logo.png" width={200} height={200} />
      </div>
    );
  }

  if (!brain.error) {
    return (
      <div className="container mx-auto p-4">
        <Alert
          variant="destructive"
          className="flex items-center justify-between"
        >
          <AlertDescription>Failed to load post.</AlertDescription>
          <Button
            variant="outline"
            size="sm"
            onClick={() => window.location.reload()}
            className="ml-4"
          >
            <RefreshCw className="h-4 w-4 mr-2" />
            Try again
          </Button>
        </Alert>
      </div>
    );
  }

  if (!brain.post) {
    return (
      <div className="container mx-auto p-4">
        <Alert>
          <AlertDescription>Post not found</AlertDescription>
        </Alert>
      </div>
    );
  }

  const calculateReadingTime = (content: string) => {
    const wordsPerMinute = 200;
    const words = content.split(/\s+/).length;
    const minutes = Math.ceil(words / wordsPerMinute);
    return `${minutes} min read`;
  };

  return (
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
            <Link href={`/post/${params.id}/edit`}>
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
  );
}
