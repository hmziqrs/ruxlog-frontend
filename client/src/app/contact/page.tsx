import {
  SiGithub,
  SiX,
  SiTelegram,
  SiGmail,
} from '@icons-pack/react-simple-icons';
import Image from 'next/image';

export default function ContactPage() {
  const links = [
    {
      icon: SiGithub,
      label: 'GitHub',
      username: '@hmziqrs',
      href: 'https://github.com/hmziqrs',
    },
    {
      icon: SiX,
      label: 'Twitter',
      username: '@hmziqrs',
      href: 'https://twitter.com/hmziqrs',
    },
    {
      icon: SiTelegram,
      label: 'Telegram',
      username: '@hmziqrs',
      href: 'https://t.me/hmziqrs',
    },
    {
      icon: SiGmail,
      label: 'Email',
      username: 'hmziqrs@gmail.com',
      href: 'mailto:hmziqrs@gmail.com',
    },
  ];

  return (
    <main className="container py-12 flex flex-col items-center">
      <section className="mt-12 flex flex-col sm:flex-row gap-x-8 gap-y-6 px-4 sm:px-0 max-w-3xl">
        <div className="min-w-[200px] self-center">
          <Image
            src="/avatar.png"
            width={200}
            height={200}
            alt="hmziqrs's profile picture"
            className="rounded-full"
          />
        </div>
        <div className="flex flex-col text-center sm:text-left">
          <h1 className="text-2xl font-semibold mb-2 font-mono">Contacts:</h1>
          <div className="flex flex-row flex-wrap gap-3">
            {links.map((link) => (
              <a
                key={link.label}
                href={link.href}
                target="_blank"
                rel="noopener noreferrer"
                aria-label={`Connect on ${link.label}`}
                className="
                    flex items-center gap-3 py-2.5 px-5 rounded-lg
                    text-sm
                    border border-zinc-200
                    transition-colors hover:bg-zinc-100 dark:border-zinc-800
                    dark:hover:bg-zinc-800"
              >
                <link.icon size={14} aria-hidden="true" />
                <span>{link.username}</span>
              </a>
            ))}
          </div>
        </div>
      </section>
    </main>
  );
}
