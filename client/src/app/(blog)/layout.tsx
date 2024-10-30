export default function Layout({ children }: { children: React.ReactNode }) {
  return (
    <>
      <header className="bg-zinc-900/50 text-white p-4">
        <div className="container mx-auto flex justify-between items-center">
          <div className="flex items-center space-x-4">
            <div className="text-lg font-bold">Logo</div>
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
      <footer className="bg-zinc-900/50 text-white p-4">
        <div className="container mx-auto flex justify-between items-center">
          <div className="flex space-x-4">
            <a href="#" className="hover:underline">
              Sitemap
            </a>
            <a href="#" className="hover:underline">
              RSS
            </a>
          </div>
          <div className="text-sm">
            &copy; {new Date().getFullYear()} Built with Tailwind CSS and
            Next.js by hmziqrs
          </div>
        </div>
      </footer>
    </>
  );
}
