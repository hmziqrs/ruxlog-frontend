import { LucideIcon } from 'lucide-react';

interface MetaPillProps {
  icon: LucideIcon;
  label: string | number;
  suffix?: string;
}

export function MetaPill({ icon: Icon, label, suffix }: MetaPillProps) {
  return (
    <span className="inline-flex items-center text-xs sm:text-sm gap-2 sm:gap-3 sm:px-4 sm:py-2 px-2.5 py-1.5 bg-zinc-100 dark:bg-zinc-800 rounded-md">
      <Icon className="w-3.5 h-3.5 sm:w-4 sm:h-4" />
      <span>
        {label}
        {suffix && ` ${suffix}`}
      </span>
    </span>
  );
}
