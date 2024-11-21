import Link from 'next/link';

const stack = [
  {
    label: 'Frontend client',
    tools: ['next.js', 'tailwindcss', 'typescript', 'react-markdown'],
  },
  {
    label: 'Admin client',
    tools: [
      'next.js',
      'tailwindcss',
      'typescript',
      'shadcn',
      'radix-ui',
      'zustand',
      'immer',
      'axios',
      'mdxeditor',
      'sonner',
    ],
  },
  {
    label: 'Backend',
    tools: [
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
    ],
  },
];

const oldRepoLinks = [
  {
    type: 'code',
    href: 'https://github.com/hmziqrs/next-blog',
    label: '@hmziqrs next-blog',
  },
  {
    type: 'data',
    href: 'https://github.com/hmziqrs/blog-posts',
    label: '@hmziqrs blog-posts',
  },
];

const sourceCodeLinks = [
  {
    type: 'backend',
    href: 'https://github.com/hmziqrs/ruxlog-backend',
    label: '@hmziqrs next-blog',
  },
  {
    type: 'admin/client',
    href: 'https://github.com/hmziqrs/ruxlog-frontend',
    label: '@hmziqrs blog-posts',
  },
];

interface ExternalLinkProps {
  href: string;
  label: string;
}

function ExternalLink({ href, label }: ExternalLinkProps) {
  return (
    <Link
      className="hover:text-zinc-300 hover:underline"
      href={href}
      aria-label={label}
      title={label}
      target="_blank"
    >
      {href}
    </Link>
  );
}

export default function AboutPage() {
  return (
    <main className="container mx-auto py-8 px-5">
      <h3 className="font-mono font-semibold text-3xl">History</h3>
      <div className="h-2" />
      <p>
        I originally wanted to build a Rust backend boilerplate with basic
        integration of common modules like authentication, database, logging,
        rate limiting, abuse limiting, account verification, etc. I have some
        ideas for projects that I want to build with Rust. However, I could not
        decide on libraries, frameworks, and structure to use. I planned to
        build a blog. When I started integrating the libraries together, the
        vision became clearer.
      </p>
      <div className="h-2" />
      <p className="">
        I always wanted to have a blog to document my learning process and
        thoughts. This is my second attempt at building a blog; the first
        attempt was to build a completely static site hosted on a GitHub
        repository using GitHub Pages. The goal was to have two repositories:
        one containing NextJS code with custom installable themes and another
        with blog content. I abandoned that project because I got hired for a
        full-time job and did not have time to work on it. It is open source and
        can be found below.
      </p>
      <div className="h-4" />
      <div className="p-4 border border-zinc-800 rounded-lg w-fit">
        <h5 className="font-mono text-xl font-semibold">Old repository</h5>
        <div className="h-1" />
        {oldRepoLinks.map(({ type, href, label }) => (
          <span key={type}>
            <span className="font-mono">{type}: </span>
            <ExternalLink href={href} label={label} />
            <br />
          </span>
        ))}
      </div>
      <div className="h-4" />
      <h3 className="font-mono font-semibold text-3xl">Goal</h3>
      <div className="h-2" />
      <p className="">
        I have a lot of experience with frontend, from development to production
        management. I have built backend servers but have little to no
        production experience. My plan is to keep iterating on this project,
        adding features, and, of course, writing blog posts.
      </p>
      <div className="h-4" />
      <h3 className="font-mono font-semibold text-3xl">Tech Stack</h3>
      <div className="h-2" />
      <div className="flex flex-col gap-y-5">
        {stack.map(({ label, tools }) => (
          <div className="mt-0" key={label}>
            <p className="font-mono font-semibold mb-2">{label}:</p>
            <div className="flex flex-wrap gap-3">
              {tools.map((tool) => (
                <span
                  key={tool}
                  title={tool}
                  className="cursor-pointer px-4 py-2 bg-zinc-100 hover:bg-zinc-100/50 dark:bg-zinc-800 hover:dark:bg-zinc-800/50 rounded-full text-sm"
                >
                  {tool}
                </span>
              ))}
            </div>
          </div>
        ))}
      </div>
      <div className="h-4" />
      <div className="p-4 border border-zinc-800 rounded-lg w-fit">
        <h5 className="font-mono text-xl font-semibold">Source code</h5>
        <div className="h-1" />
        {sourceCodeLinks.map(({ type, href, label }) => (
          <span key={type}>
            <span className="font-mono">{type}: </span>
            <ExternalLink href={href} label={label} />
            <br />
          </span>
        ))}
      </div>
    </main>
  );
}
