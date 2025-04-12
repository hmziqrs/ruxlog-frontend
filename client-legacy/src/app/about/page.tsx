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
        I wanted to build a Rust backend boilerplate with common features like
        auth and database. I had ideas for Rust projects but was not sure which
        tools to use. Building this blog helped me figure out the right
        approach.
      </p>
      <div className="h-2" />
      <p className="">
        I wanted a blog to share what I learn and think about. My first try was
        a static site on GitHub Pages with two repos - one for code, one for
        content. I stopped working on it when I got a full-time job, but the
        code is still available below.
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
      <div className="h-8" />
      <h3 className="font-mono font-semibold text-3xl">Goal</h3>
      <div className="h-2" />
      <p className="">
        I am experienced with frontend development and managing production
        sites. While I have built backend servers, I do not have much real-world
        experience with them. I want to grow this project by adding features and
        sharing through blog posts.
      </p>
      <div className="h-8" />
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
