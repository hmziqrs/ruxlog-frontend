import { Loader, Loader2 } from 'lucide-react';
import Image from 'next/image';

export function ContentLoader({ text = 'Loading...' }: { text: string }) {
  return (
    <div className="flex justify-center items-center flex-col">
      <div className="flex flex-col items-center gap-6">
        <Image
          alt="Logo"
          src="/logo.png"
          width={190}
          height={190}
          className="animate-pulse"
        />
        <Loader2 className="h-14 w-14 animate-spin text-primary" />
        <p className="text-xl font-medium text-muted-foreground animate-fade-in">
          {text}
        </p>
      </div>
    </div>
  );
}
