'use client';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select';
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
import {
  LayoutGrid,
  List,
  Loader2,
  Pencil,
  Plus,
  Trash,
  Eye,
} from 'lucide-react';

const PostItem = ({ post, brain }: { post: Post; brain: PostBrain }) => {
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
      <CardContent className="p-0">
        <div className="flex flex-col">
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
            <Badge
              variant={post.isPublished ? 'default' : 'secondary'}
              className="absolute bottom-2 left-2 shadow-md cursor-pointer"
            >
              {post.isPublished ? 'Published' : 'Draft'}
            </Badge>
          </div>

          {/* Content Section */}
          <div className="flex-1 p-4">
            <div className="mb-4 flex items-start gap-2">
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
            <div className="space-y-2 text-sm text-muted-foreground">
              <div className="flex flex-wrap gap-2">
                <span className="flex items-center gap-1">
                  <span className="font-medium">Author:</span>
                  {post.author?.name || 'Anonymous'}
                </span>
                <span>•</span>
                <span className="flex items-center gap-1">
                  <span className="font-medium">Category:</span>
                  {post.category?.name || 'Uncategorized'}
                </span>
              </div>

              <div className="flex flex-wrap gap-x-4 gap-y-1">
                <span className="flex items-center gap-1">
                  <span className="font-medium">Created:</span>
                  {formatDate(post.createdAt)}
                </span>
                <span className="flex items-center gap-1">
                  <span className="font-medium">Updated:</span>
                  {formatDate(post.updatedAt)}
                </span>
                {post.publishedAt && (
                  <span className="flex items-center gap-1">
                    <span className="font-medium">Published:</span>
                    {formatDate(post.publishedAt)}
                  </span>
                )}
              </div>
              <div className="flex flex-grow" />

              {/* Tags */}
              {post.tagIds && post.tagIds.length > 0 && (
                <div className="flex flex-wrap gap-2">
                  {post.tagIds.map((tag) => (
                    <span
                      key={tag}
                      className="rounded-full bg-secondary px-2 py-1 text-xs"
                    >
                      {tag}
                    </span>
                  ))}
                </div>
              )}
            </div>

            {/* Actions */}
            <div className="mt-4 flex items-center justify-between gap-2">
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
                  <Link href={`/preview/${post.id}`}>
                    <Eye className="h-4 w-4" />
                    <span className="hidden sm:inline ml-2">Preview</span>
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

export default function PostPage() {
  const brain = usePostBrain();

  return (
    <div className="px-4 sm:px-6 lg:px-8 py-8">
      <div className="max-w-7xl mx-auto">
        <h1 className="text-3xl font-bold">Posts</h1>
        <Link href="/posts/new">
          <Button>
            <Plus className="mr-2 h-4 w-4" /> New Post
          </Button>
        </Link>
      </div>

      <div className="mb-6 flex items-center gap-4">
        <Input
          placeholder="Search posts..."
          className="max-w-sm"
          value={brain.filters.search}
          // onChange={(e) => brain.handleSearch(e.target.value)}
        />
        <Select
          value={brain.filters.sortBy}
          // onValueChange={(value) => brain.handleSort(value as any)}
        >
          <SelectTrigger className="w-[180px]">
            <SelectValue placeholder="Sort by..." />
          </SelectTrigger>
          <SelectContent>
            <SelectItem value="Title">Title</SelectItem>
            <SelectItem value="UpdatedAt">Updated At</SelectItem>
            <SelectItem value="PublishedAt">Published At</SelectItem>
            <SelectItem value="ViewCount">View Count</SelectItem>
            <SelectItem value="LikesCount">Likes Count</SelectItem>
          </SelectContent>
        </Select>
        <div className="flex-1" />
        <Button
          variant="outline"
          size="icon"
          onClick={() => brain.setViewMode('grid')}
          className={brain.viewMode === 'grid' ? 'bg-accent' : ''}
        >
          <LayoutGrid className="h-4 w-4" />
        </Button>
        <Button
          variant="outline"
          size="icon"
          onClick={() => brain.setViewMode('list')}
          className={brain.viewMode === 'list' ? 'bg-accent' : ''}
        >
          <List className="h-4 w-4" />
        </Button>
        {brain.selectedPosts.length > 0 && (
          <Button
            variant="destructive"
            // onClick={brain.handleDelete}
          >
            <Trash className="mr-2 h-4 w-4" />
            Delete Selected
          </Button>
        )}
      </div>

      {brain.loading ? (
        <div className="flex justify-center py-8">
          <Loader2 className="h-8 w-8 animate-spin" />
        </div>
      ) : (
        <div
          className={
            brain.viewMode === 'grid'
              ? 'grid grid-cols-1 gap-4 sm:grid-cols-2 lg:grid-cols-3'
              : 'flex flex-col gap-4'
          }
        >
          {brain.posts.map((post) => (
            <PostItem key={post.id} post={post} brain={brain} />
          ))}
        </div>
      )}
    </div>
  );
}
