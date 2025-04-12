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
import { Badge } from '@/components/ui/badge';
import { Category } from '@/store/category/types';
import { CategoryBrain, useCategoryItemBrain } from './brain';
import { Card, CardContent } from '@/components/ui/card';
import Link from 'next/link';
import { Pencil, Trash, Folder, Eye } from 'lucide-react';
import { ContentLoader } from '@/components/content-loader';

export const CategoryItem = ({
  category,
}: {
  category: Category;
  brain: CategoryBrain;
}) => {
  const categoryItemBrain = useCategoryItemBrain(category.id);

  return (
    <Card className="relative overflow-hidden">
      <CardContent className="flex flex-grow w-full h-full overflow-hidden p-0 bg-zinc-900/50">
        <div className="flex flex-col w-full ">
          {/* Image Section */}
          <div className="relative h-48 w-full">
            <img
              src={
                category.coverImage ||
                'https://placehold.co/600x400/e2e8f0/94a3b8'
              }
              alt={category.name}
              className="h-full w-full object-cover"
            />
          </div>

          {/* Content Section */}
          <div className="flex-1 p-4 flex-col">
            <h3 className="font-semibold line-clamp-1">{category.name}</h3>
            <p className="mt-2 text-sm text-muted-foreground line-clamp-2">
              {category.description}
            </p>

            {/* Meta Information */}
            <div className="flex-row gap-2 mt-4">
              <Badge
                variant="secondary"
                className="font-semibold py-2 px-4 gap-2 mr-3 mb-3"
              >
                <Folder className="h-4 w-4" />
                {category.parentId
                  ? `Parent ID: ${category.parentId}`
                  : 'No Parent'}
              </Badge>
            </div>
          </div>
          <div className="flex items-center justify-between gap-2 px-4 mb-4">
            {/* Action Buttons */}
            <div className="">
              <div className="flex items-center gap-2">
                <Link href={`/category/update/${category.id}`}>
                  <Button size="sm" variant="outline">
                    <Pencil className="h-4 w-4" />
                    <span className="hidden sm:inline ml-2">Edit</span>
                  </Button>
                </Link>

                <Button size="sm" variant="outline" asChild>
                  <Link href={`/category/view/${category.id}`}>
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
                        delete &quot;{category.name}&quot; and remove it from
                        our servers.
                      </AlertDialogDescription>
                    </AlertDialogHeader>
                    <AlertDialogFooter>
                      <AlertDialogCancel>Cancel</AlertDialogCancel>
                      <AlertDialogAction
                        className="bg-destructive text-destructive-foreground hover:bg-destructive/90"
                        onClick={() => categoryItemBrain.removeCategory()}
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
      {categoryItemBrain.loading && <ContentLoader absolute size="sm" />}
    </Card>
  );
};
