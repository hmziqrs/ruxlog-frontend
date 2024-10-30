import Link from 'next/link';

export default function Layout({ children }: { children: React.ReactNode }) {
  return (
    <>
      <header className="bg-zinc-900/50 p-4">
        <div className="container mx-auto flex justify-between items-center">
          <div className="flex items-center space-x-4">
            <div className="text-lg">Geist</div>
            <div className="flex space-x-4">
              <a href="#" className="hover:underline">
                About
              </a>
              <a href="#" className="hover:underline">
                Contact
              </a>
            </div>
          </div>
          <div className="flex items-center space-x-4">
            <div className="text-lg">Profile Icon</div>
          </div>
        </div>
      </header>
      <div className="flex flex-grow container mx-auto">{children}</div>
      <footer className="bg-zinc-900/50 p-8">
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
