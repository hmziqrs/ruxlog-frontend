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
import { Post } from '@/store/post/types';
import { usePostBrain, PostBrain } from './brain';
import { Card, CardContent } from '@/components/ui/card';
import { Checkbox } from '@/components/ui/checkbox';
import Link from 'next/link';
import { LayoutGrid, List, Loader2, Pencil, Plus, Trash } from 'lucide-react';

const PostItem = ({ post, brain }: { post: Post; brain: PostBrain }) => {
  const isSelected = brain.selectedPosts.includes(post.id);

  return (
    <Card className="relative">
      <CardContent className="p-4">
        <div className="flex items-start gap-4">
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
            <h3 className="font-semibold">{post.title}</h3>
            <p className="text-sm text-gray-500">{post.excerpt}</p>
            <div className="mt-2 flex items-center gap-2">
              <Button
                variant={post.isPublished ? 'default' : 'outline'}
                size="sm"
                // onClick={() => brain.handleTogglePublish(post.id)}
              >
                {post.isPublished ? 'Published' : 'Draft'}
              </Button>
              <Link href={`/posts/${post.id}/edit`}>
                <Button size="sm" variant="ghost">
                  <Pencil className="h-4 w-4" />
                </Button>
              </Link>
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
    <div className="container py-8">
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
