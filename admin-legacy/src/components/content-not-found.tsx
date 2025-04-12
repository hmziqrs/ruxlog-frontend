import {
  AlertTriangle,
  ArrowLeft,
  FileQuestion,
  RefreshCw,
} from 'lucide-react';
import { Card } from './ui/card';
import { Button } from './ui/button';
import Link from 'next/link';

interface Props {
  title: string;
  desc: string;
  onRetry?: () => void;
  onBack?: () => void;
  onBackLabel: string;
  onBackLink?: string;
}

export function ContentNotFound({
  title = 'Not found',
  desc = "The content you're looking for doesn't exist or has been removed.",
  onBack,
  onBackLabel,
  onBackLink,
}: Props) {
  return (
    <div className="flex justify-center items-center min-h-[80vh]">
      <div className="max-w-md w-full">
        <Card className="p-6">
          <div className="flex flex-col items-center gap-6 text-center">
            <div className="h-20 w-20 rounded-full bg-muted flex items-center justify-center">
              <FileQuestion className="h-10 w-10 text-muted-foreground" />
            </div>
            <div className="space-y-2">
              <h2 className="text-2xl font-bold tracking-tight">{title}</h2>
              <p className="text-muted-foreground">{desc}</p>
            </div>
            <Button variant="default" onClick={onBack} asChild={!!onBackLink}>
              <Link href={onBackLink ?? {}}>
                <ArrowLeft className="h-4 w-4 mr-2" />
                {onBackLabel}
              </Link>
            </Button>
          </div>
        </Card>
      </div>
    </div>
  );
}
