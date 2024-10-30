import Link from 'next/link';
import Image from 'next/image';
import { ThemeToggle } from '@/components/theme-toggle';

export default function Layout({ children }: { children: React.ReactNode }) {
  return (
    <>
      <header className="dark:bg-zinc-900/50 p-4">
        <div className="container mx-auto flex items-center">
          <Link href="/" className="flex justify-center items-center">
            <Image src="/logo.png" width={60} height={60} alt="logo" />
          </Link>
          <div className="flex-grow" />
          <div className="space-x-4 text-md">
            {/* <Link href="/" className="hover:underline">
              Home
            </Link> */}
            <Link href="/about" className="hover:underline">
              About
            </Link>
            <Link href="/contact" className="hover:underline">
              Contact
            </Link>
            <span>Profile Icon</span>
            <ThemeToggle />
          </div>
        </div>
      </header>
      <div className="flex flex-grow container mx-auto">{children}</div>
      <footer className="dark:bg-zinc-900/50 p-8">
        <div className="container mx-auto flex flex-col items-center">
          <div className="font-mono text-sm">
            &copy; {new Date().getFullYear()}, Built with Tailwind CSS, Next.js
            and ❤️ by hmziqrs
          </div>
          <div className="h-4" />
          <div className="space-x-4 text-xs dark:text-zinc-700">
            <Link href="#" className="hover:underline">
              Privacy Policy
            </Link>
            <Link href="#" className="hover:underline">
              Terms of Service
            </Link>
            <Link href="#" className="hover:underline">
              Sitemap
            </Link>
            <Link href="#" className="hover:underline">
              RSS
            </Link>
          </div>
        </div>
      </footer>
    </>
  );
}
