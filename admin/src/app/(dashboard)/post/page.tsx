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
import { usePostBrain } from './brain';
import Link from 'next/link';
import { LayoutGrid, List, Loader2, Plus, Trash, Eye } from 'lucide-react';
import { PostItem } from './_post-item';

export default function PostPage() {
  const brain = usePostBrain();

  return (
    <div className="px-4 sm:px-6 lg:px-8 py-8">
      <div className="max-w-7xl mx-auto">
        <h1 className="text-3xl font-bold">Posts</h1>
        <Link href="/post/create">
          <Button>
            <Plus className="mr-2 h-4 w-4" /> Create post
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
