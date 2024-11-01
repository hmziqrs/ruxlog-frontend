import Link from 'next/link';

export default function AboutPage() {
  const frontendTools = [
    'next.js',
    'tailwindcss',
    'typescript',
    'react-markdown',
  ];
  const adminTools = [
    'next.js',
    'tailwindcss',
    'typescript',
    'shadcn',
    'zustand',
    'immer',
    'axios',
    'mdxeditor',
    'sonner',
  ];

  const backendTools = [
    'rust',
    'axum',
    'axum-login',
    'tower-http',
    'validator',
    'serde',
    'lettre',
    'diesel',
    'postgres',
    'redis',
  ];

  return (
    <main className="container mx-auto py-8 px-5">
      <h3 className="font-mono font-semibold text-3xl">History</h3>
      <div className="h-2" />
      <p>
        I originally wanted to build a rust backend boilerplate with basic
        integration of common modules like authentication, database, logging,
        rate limiting, abuse limiting, account verification, etc. I have some
        ideas for projects which I want to build with rust. However I could not
        decide on libraries, framework and structure to use. I planned to build
        a blog. When I started integrating the vision I have became more clear.
      </p>
      <div className="h-2" />
      <p className="">
        I always wanted to have a blog where I can document my learning process
        and my thoughts. Well this is my second attempt at building a blog,
        first attempt was to build a completely static and hosted on a github
        repository using github pages. The goal was two have two repositories a
        contains next code with custom installable themes and another with
        contents of the blog. I abandoned that project because I got hired for a
        full time job and I did not have time to work on it. It is open source
        can be found below.
      </p>
      <div className="h-4" />
      <div className="p-4 border border-zinc-800 rounded-lg w-fit">
        <span>
          <h5 className="font-mono text-xl font-semibold">Old repository</h5>
          <div className="h-1" />
          <span className="font-mono">code: </span>
          <Link
            className="hover:text-zinc-300 hover:underline"
            href="https://github.com/hmziqrs/next-blog"
            target="_blank"
          >
            https://github.com/hmziqrs/next-blog
          </Link>
        </span>
        <br />
        <span>
          <span className="font-mono">data: </span>
          <Link
            className="hover:text-zinc-300 hover:underline"
            href="https://github.com/hmziqrs/blog-posts"
            target="_blank"
          >
            https://github.com/hmziqrs/blog-posts
          </Link>
        </span>
      </div>
      <div className="h-4" />
      <h3 className="font-mono font-semibold text-3xl">Goal</h3>
      <div className="h-2" />
      <p className="">
        I have a lot of experience with frontend from development to production
        management. I have built backend servers but little to none production
        experience. My plan is keep iterating on this project and add features.
        and off course write blog posts.
      </p>
      <div className="h-4" />
      <h3 className="font-mono font-semibold text-3xl">Tech Stack</h3>
      <div className="h-2" />
      <div>
        <p className="font-mono font-semibold mb-2">Frontend client:</p>
        <div className="flex flex-wrap gap-3">
          {frontendTools.map((tool) => (
            <span
              key={tool}
              className="cursor-pointer px-4 py-2 bg-zinc-800 rounded-full text-sm"
            >
              {tool}
            </span>
          ))}
        </div>
      </div>
      <div className="h-4" />
      <div>
        <p className="font-mono font-semibold mb-2">Admin client:</p>
        <div className="flex flex-wrap gap-3">
          {adminTools.map((tool) => (
            <span
              key={tool}
              className="cursor-pointer px-4 py-2 bg-zinc-800 rounded-full text-sm"
            >
              {tool}
            </span>
          ))}
        </div>
      </div>
      <div className="h-4" />
      <div>
        <p className="font-mono font-semibold mb-2">Backend:</p>
        <div className="flex flex-wrap gap-3">
          {backendTools.map((tool) => (
            <span
              key={tool}
              className="cursor-pointer px-4 py-2 bg-zinc-800 rounded-full text-sm"
            >
              {tool}
            </span>
          ))}
        </div>
      </div>
    </main>
  );
}
