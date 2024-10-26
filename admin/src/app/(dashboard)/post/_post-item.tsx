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
import { Switch } from '@/components/ui/switch';
import { Badge } from '@/components/ui/badge';
import { Post } from '@/store/post/types';
import { usePostBrain, PostBrain } from './brain';
import { Card, CardContent } from '@/components/ui/card';
import { Checkbox } from '@/components/ui/checkbox';
import Link from 'next/link';
import { Pencil, Trash, Eye, Heart, Folder, User, User2 } from 'lucide-react';

export const PostItem = ({ post, brain }: { post: Post; brain: PostBrain }) => {
  const isSelected = brain.selectedPosts.includes(post.id);

  const formatDate = (date: string) => {
    return new Date(date).toLocaleDateString('en-US', {
      year: 'numeric',
      month: 'short',
      day: 'numeric',
    });
  };

  return (
    <Card className="relative overflow-hidden">
      <CardContent className="flex flex-grow w-full h-full overflow-hidden p-0">
        <div className="flex flex-col w-full ">
          {/* Image Section with Status Badge */}
          <div className="relative h-48 w-full">
            <img
              src={
                post.featuredImageUrl ||
                'https://placehold.co/600x400/e2e8f0/94a3b8'
              }
              alt={post.title}
              className="h-full w-full object-cover"
            />
          </div>

          {/* Content Section */}
          <div className="flex-1 p-4 flex-col">
            <div className="flex items-start gap-2">
              <Checkbox
                checked={isSelected}
                onCheckedChange={(checked) => {
                  brain.setSelectedPosts(
                    checked
                      ? [...brain.selectedPosts, post.id]
                      : brain.selectedPosts.filter((id) => id !== post.id)
                  );
                }}
              />
              <div className="flex-1">
                <h3 className="font-semibold line-clamp-1">{post.title}</h3>
                <p className="mt-2 text-sm text-muted-foreground line-clamp-2">
                  {post.excerpt}
                </p>
              </div>
            </div>

            {/* Meta Information */}
            <div className="flex-row gap-2">
              <Badge
                variant={post.isPublished ? 'default' : 'secondary'}
                className="font-semibold py-2 px-4 gap-2 mr-3 mb-3"
              >
                <Folder className="h-4 w-4" />

                <span>{post.isPublished ? 'Published' : 'Draft'}</span>
              </Badge>
              <Badge
                variant="secondary"
                className="font-semibold py-2 px-4 gap-2 mr-3 mb-3"
              >
                <Folder className="h-4 w-4" />
                {post.category?.name || 'Uncategorized'}f
              </Badge>
              <Badge
                variant="secondary"
                className="font-semibold py-2 px-4 gap-2 mr-3 mb-3"
              >
                <span className="flex items-center gap-1">
                  <User2 className="h-4 w-4" />
                  {post.author?.name || 'Anonymous'}
                </span>
              </Badge>
              <Badge
                variant="secondary"
                className="font-semibold py-2 px-4 gap-2 mr-3 mb-3"
              >
                <span className="flex items-center gap-1">
                  <Eye className="h-4 w-4" />
                  {post.viewCount} views
                </span>
              </Badge>
              <Badge
                variant="secondary"
                className="font-semibold py-2 px-4 gap-2 mr-3 mb-3"
              >
                <span className="flex items-center gap-1">
                  <Heart className="h-4 w-4" />
                  {post.likesCount} likes
                </span>
              </Badge>
            </div>
          </div>
          <div className="flex items-center justify-between gap-2 px-4 mb-4">
            {/* Publish Toggle */}
            <div className="flex items-center gap-2">
              <Switch
                checked={post.isPublished}
                onCheckedChange={() => brain.handleTogglePublish?.(post.id)}
                aria-label="Toggle publish status"
              />
              <span className="text-sm text-muted-foreground">
                {post.isPublished ? 'Unpublish' : 'Publish'}
              </span>
            </div>

            {/* Action Buttons */}
            <div className="px-4">
              <div className="flex items-center gap-2">
                <Link href={`/posts/${post.id}/edit`}>
                  <Button size="sm" variant="outline">
                    <Pencil className="h-4 w-4" />
                    <span className="hidden sm:inline ml-2">Edit</span>
                  </Button>
                </Link>

                <Button
                  size="sm"
                  variant="outline"
                  className="text-muted-foreground"
                  asChild
                >
                  <Link href={`/post/view/${post.id}`}>
                    <Eye className="h-4 w-4" />
                    <span className="hidden sm:inline ">View</span>
                  </Link>
                </Button>

                <AlertDialog>
                  <AlertDialogTrigger asChild>
                    <Button size="sm" variant="destructive">
                      <Trash className="h-4 w-4" />
                      <span className="hidden sm:inline ml-2">Delete</span>
                    </Button>
                  </AlertDialogTrigger>
                  <AlertDialogContent>
                    <AlertDialogHeader>
                      <AlertDialogTitle>
                        Are you absolutely sure?
                      </AlertDialogTitle>
                      <AlertDialogDescription>
                        This action cannot be undone. This will permanently
                        delete &quot;{post.title}&quot; and remove it from our
                        servers.
                      </AlertDialogDescription>
                    </AlertDialogHeader>
                    <AlertDialogFooter>
                      <AlertDialogCancel>Cancel</AlertDialogCancel>
                      <AlertDialogAction
                        className="bg-destructive text-destructive-foreground hover:bg-destructive/90"
                        onClick={() => brain.handleDelete?.(post.id)}
                      >
                        Delete
                      </AlertDialogAction>
                    </AlertDialogFooter>
                  </AlertDialogContent>
                </AlertDialog>
              </div>
            </div>
          </div>
        </div>
      </CardContent>
    </Card>
  );
};
