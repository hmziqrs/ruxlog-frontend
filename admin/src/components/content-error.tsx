import { AlertTriangle, ArrowLeft, RefreshCw } from 'lucide-react';
import { Card } from './ui/card';
import { Button } from './ui/button';
import Link from 'next/link';

interface Props {
  title: string;
  desc?: string;
  onRetry?: () => void;
  onBack?: () => void;
  onBackLabel?: string;
  onBackLink?: string;
}

export function ContentError({
  title = 'Failed',
  desc = 'There was an error. please again',
  onRetry,
  onBack,
  onBackLabel,
  onBackLink,
}: Props) {
  return (
    <div className="flex justify-center items-center ">
      <div className="max-w-md w-full">
        <Card className="p-6">
          <div className="flex flex-col items-center gap-6 text-center">
            <div className="h-20 w-20 rounded-full bg-destructive/10 flex items-center justify-center">
              <AlertTriangle className="h-10 w-10 text-destructive" />
            </div>
            <div className="space-y-2">
              <h2 className="text-2xl font-bold tracking-tight">{title}</h2>
              <p className="text-muted-foreground">{desc}</p>
            </div>
            <div className="flex gap-4">
              <Button variant="outline" onClick={onRetry} className="gap-2">
                <RefreshCw className="h-4 w-4" />
                Try again
              </Button>
              {(onBack || onBackLink) && (
                <Button
                  variant="default"
                  onClick={onBack}
                  asChild={!!onBackLink}
                >
                  <Link href={onBackLink ?? {}}>
                    <ArrowLeft className="h-4 w-4 mr-2" />
                    {onBackLabel}
                  </Link>
                </Button>
              )}
            </div>
          </div>
        </Card>
      </div>
    </div>
  );
}
