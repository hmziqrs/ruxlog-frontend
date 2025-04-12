'use client';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { useCategoryBrain } from './brain';
import Link from 'next/link';
import { Loader2, Plus } from 'lucide-react';
import { CategoryItem } from './_category-item';

export default function CategoryPage() {
  const brain = useCategoryBrain();

  return (
    <div className="px-4 sm:px-6 lg:px-8 py-8">
      <div className="mb-6 flex items-center gap-4">
        <Input
          placeholder="Search categories..."
          className="max-w-sm"
          // onChange={(e) => brain.handleSearch(e.target.value)}
        />
        <div className="flex-1" />
        <Link href="/category/create">
          <Button>
            <Plus className="mr-2 h-4 w-4" /> Create category
          </Button>
        </Link>
      </div>

      {brain.loading ? (
        <div className="flex justify-center py-8">
          <Loader2 className="h-8 w-8 animate-spin" />
        </div>
      ) : (
        <div
          className={'grid gap-4 sm:grid-cols-1 md:grid-cols-2 lg:grid-cols-3'}
        >
          {brain.categories.map((category) => (
            <CategoryItem key={category.id} category={category} brain={brain} />
          ))}
        </div>
      )}
    </div>
  );
}
