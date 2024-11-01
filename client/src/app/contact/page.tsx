import {
  SiGithub,
  SiX,
  SiLinkedin,
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
      icon: SiLinkedin,
      label: 'LinkedIn',
      username: 'hmziqrs',
      href: 'https://linkedin.com/in/hmziqrs',
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
    <div className="container py-10 flex flex-col items-center">
      <div className="text-center max-w-lg">
        {/* <h1 className="text-4xl font-semibold tracking-tighter ">
          Like my work?
        </h1>
        <div className="h-1" /> */}
        <p className="font-mono text-2xl sm:text-3xl font-semibold text-zinc-800 dark:text-zinc-200">
          looking for an engineer?
        </p>
        <div className="h-2" />
        <p className=" text-zinc-500 dark:text-zinc-200 px-4">
          I am always looking to work on exciting projects where I can utilize
          my skills to build and ship software
        </p>
      </div>
      <div className="h-12" />
      <div className="flex flex-col sm:flex-row gap-x-6 gap-y-4 px-4 sm:px-0 max-w-3xl">
        <div className="min-w-[200px] self-center">
          <Image
            src="/avatar.png"
            width={200}
            height={200}
            alt="hmziqrs's avatar"
            className="rounded-full"
          />
        </div>
        <div className="flex flex-col text-center sm:text-left">
          <p>
            Full stack engineer with 7+ years of experience in building
            mobile/web apps and backend.
          </p>
          <p>You can find me @hmziqrs</p>
          <div className="h-4" />
          <div className="flex flex-row flex-wrap gap-3">
            {links.map((link) => (
              <a
                key={link.label}
                href={link.href}
                target="_blank"
                rel="noopener noreferrer"
                className="
                    flex items-center gap-3 py-2.5 px-5 rounded-lg
                    text-sm
                    border border-zinc-200 
                    transition-colors hover:bg-zinc-100 dark:border-zinc-800 
                    dark:hover:bg-zinc-800"
              >
                <link.icon size={14} />
                <span className="">{link.username}</span>
              </a>
            ))}
          </div>
        </div>
      </div>
    </div>
  );
}
