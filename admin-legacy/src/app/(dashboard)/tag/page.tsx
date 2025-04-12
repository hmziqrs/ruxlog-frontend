'use client';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { useTagBrain } from './brain';
import Link from 'next/link';
import { Loader2, Plus } from 'lucide-react';
import { TagItem } from './_tag-item';

export default function TagPage() {
  const brain = useTagBrain();

  return (
    <div className="px-4 sm:px-6 lg:px-8 py-8">
      <div className="mb-6 flex items-center gap-4">
        <Input
          placeholder="Search tags..."
          className="max-w-sm"
          // onChange={(e) => brain.handleSearch(e.target.value)}
        />
        <div className="flex-1" />
        <Link href="/tag/create">
          <Button>
            <Plus className="mr-2 h-4 w-4" /> Create tag
          </Button>
        </Link>
      </div>

      {brain.loading ? (
        <div className="flex justify-center py-8">
          <Loader2 className="h-8 w-8 animate-spin" />
        </div>
      ) : (
        <div className="overflow-x-auto">
          <table className="min-w-full bg-zinc-900/50 border border-zinc-800 rounded-lg">
            <thead>
              <tr>
                <th className="px-6 py-3 text-left text-xs font-medium text-zinc-400 uppercase tracking-wider">
                  Name
                </th>
                <th className="px-6 py-3 text-left text-xs font-medium text-zinc-400 uppercase tracking-wider">
                  Slug
                </th>
                <th className="px-6 py-3 text-left text-xs font-medium text-zinc-400 uppercase tracking-wider">
                  Description
                </th>
                <th className="px-6 py-3 text-left text-xs font-medium text-zinc-400 uppercase tracking-wider">
                  Created At
                </th>
                <th className="px-6 py-3 text-left text-xs font-medium text-zinc-400 uppercase tracking-wider">
                  Updated At
                </th>
                <th className="px-6 py-3 text-left text-xs font-medium text-zinc-400 uppercase tracking-wider">
                  Actions
                </th>
              </tr>
            </thead>
            <tbody className="divide-y divide-zinc-800">
              {brain.tags.map((tag) => (
                <TagItem key={tag.id} tag={tag} brain={brain} />
              ))}
            </tbody>
          </table>
        </div>
      )}
    </div>
  );
}
