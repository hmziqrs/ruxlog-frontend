import { Loader, Loader2 } from 'lucide-react';
import Image from 'next/image';

const sizes = {
  sm: { logo: 100, fontSize: 'text-sm', spinner: 'h-8 w-8' },
  md: { logo: 140, fontSize: 'text-lg', spinner: 'h-11 w-11' },
  lg: { logo: 180, fontSize: 'text-lg', spinner: 'h-14 w-14' },
  xl: { logo: 220, fontSize: 'text-lg', spinner: 'h-18 w-18' },
};

export function ContentLoader({
  text,
  size = 'md',
  absolute = true,
}: {
  text?: string;
  size?: keyof typeof sizes;
  absolute?: boolean;
}) {
  const currentSize = sizes[size];
  const content = (
    <div className="flex flex-col items-center gap-6">
      <Image
        alt="Logo"
        src="/logo.png"
        width={currentSize.logo}
        height={currentSize.logo}
        className="animate-pulse"
      />
      <Loader2 className={`${currentSize.spinner} animate-spin text-primary`} />
      {text && (
        <p
          className={`${currentSize.fontSize} font-medium text-muted-foreground animate-fade-in`}
        >
          {text}
        </p>
      )}
    </div>
  );

  if (absolute) {
    return (
      <div className="absolute inset-0 flex items-center justify-center bg-zinc-950 bg-opacity-50">
        {content}
      </div>
    );
  }

  return content;
}
