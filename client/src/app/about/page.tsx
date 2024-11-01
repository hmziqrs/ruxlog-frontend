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
        decide on libraries, frameworks, and structure. I planned to build a
        blog. When I started integrating the libraries together the vision
        became clearer.
      </p>
      <div className="h-2" />
      <p className="">
        I always wanted to have a blog to document my learning process and
        thoughts. Well, this is my second attempt at building a blog, the first
        attempt was to build a completely static and hosted on a GitHub
        repository using GitHub pages. The goal was to have two repositories
        containing NextJS code with custom installable themes and another with
        blog content. I abandoned that project because I got hired for a
        full-time job and I did not have time to work on it. It is open source
        and can be found below.
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
            aria-label="@hmziqrs next-blog"
            title="@hmziqrs next-blog"
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
            aria-label="@hmziqrs blog-posts"
            title="@hmziqrs blog-posts"
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
        management. I have built backend servers but little to no production
        experience. My plan is to keep iterating on this project and add
        features. and off course write blog posts.
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
    </main>
  );
}
