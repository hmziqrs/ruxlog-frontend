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
import { LayoutGrid, List, Loader2, Pencil, Plus, Trash } from 'lucide-react';
const PostItem = ({ post, brain }: { post: Post; brain: PostBrain }) => {
  // ... previous code remains the same until the Actions section ...

  // Actions section replacement
  const renderActions = () => (
    <div className="mt-4 flex items-center justify-between">
      {/* Status and Publish Toggle */}
      <div className="flex items-center gap-2">
        <Badge
          variant={post.isPublished ? 'default' : 'secondary'}
          className="h-6 cursor-pointer"
        >
          {post.isPublished ? 'Published' : 'Draft'}
        </Badge>
        <div className="flex items-center gap-2">
          <Switch
            checked={post.isPublished}
            // onCheckedChange={() => brain.handleTogglePublish?.(post.id)}
            aria-label="Toggle publish status"
          />
          <span className="text-sm text-muted-foreground">
            {post.isPublished ? 'Unpublish' : 'Publish'}
          </span>
        </div>
      </div>

      {/* Action Buttons */}
      <div className="flex items-center gap-2">
        <Link href={`/posts/${post.id}/edit`}>
          <Button size="sm" variant="outline">
            <Pencil className="mr-2 h-4 w-4" />
            Edit
          </Button>
        </Link>

        <Button
          size="sm"
          variant="outline"
          className="text-muted-foreground"
          asChild
        >
          <Link href={`/preview/${post.id}`}>Preview</Link>
        </Button>

        <AlertDialog>
          <AlertDialogTrigger asChild>
            <Button size="sm" variant="destructive">
              <Trash className="mr-2 h-4 w-4" />
              Delete
            </Button>
          </AlertDialogTrigger>
          <AlertDialogContent>
            <AlertDialogHeader>
              <AlertDialogTitle>Are you absolutely sure?</AlertDialogTitle>
              <AlertDialogDescription>
                This action cannot be undone. This will permanently delete
                &quot;{post.title}&quot; and remove it from our servers.
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
  );

  return (
    <Card className="relative overflow-hidden">
      <CardContent className="p-0">
        <div className="flex flex-col">
          {/* Image Section */}
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
          <div className="flex-1 p-4">
            <div className="mb-4 flex items-start justify-between">
              <div className="flex-1">
                <div className="flex items-center gap-2">
                  <Checkbox
                    // checked={isSelected}
                    onCheckedChange={(checked) => {
                      brain.setSelectedPosts(
                        checked
                          ? [...brain.selectedPosts, post.id]
                          : brain.selectedPosts.filter((id) => id !== post.id)
                      );
                    }}
                  />
                  <h3 className="font-semibold line-clamp-1">{post.title}</h3>
                </div>
                <p className="mt-2 text-sm text-muted-foreground line-clamp-2">
                  {post.excerpt}
                </p>
              </div>
            </div>

            {/* Meta Information */}
            {/* ... previous meta information code remains the same ... */}

            {/* Actions */}
            {renderActions()}
          </div>
        </div>
      </CardContent>
    </Card>
  );
};

export default function PostPage() {
  const brain = usePostBrain();

  return (
    <div className=" py-8">
      <div className="mb-8 flex items-center justify-between">
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
