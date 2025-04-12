import Link from 'next/link';
import Image from 'next/image';
import { ThemeToggle } from './theme-toggle';

export function LayoutHeader() {
  return (
    <header className="dark:bg-zinc-900/50 bg-zinc-100/50 p-4">
      <div className="container mx-auto flex items-center justify-center">
        <Link href="/" className="flex justify-center items-center">
          <Image src="/logo.png" width={60} height={60} alt="logo" />
        </Link>
        <div className="flex-grow" />
        <div className="space-x-4 text-md flex items-center">
          <Link href="/about" className="hover:underline">
            About
          </Link>
          <Link href="/contact" className="hover:underline">
            Contact
          </Link>

          {/* <User size={20} className="cursor-pointer hover:text-zinc-400 " /> */}
          <ThemeToggle className="cursor-pointer hover:text-zinc-400 " />
        </div>
      </div>
    </header>
  );
}
