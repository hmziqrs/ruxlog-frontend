import {
  SiGithub,
  SiX,
  SiLinkedin,
  SiTelegram,
  SiGmail,
} from '@icons-pack/react-simple-icons';

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
    <div className="container py-10">
      <div className="flex flex-col items-center justify-center space-y-8">
        
        <div className="text-center space-y-2">
          <h1 className="text-3xl font-bold tracking-tighter sm:text-4xl md:text-5xl">
            Lets Connect
          </h1>
          <p className="text-zinc-500 dark:text-zinc-400">
            Feel free to reach out through any of these platforms
          </p>
        </div>

        <div className="grid w-full grid-cols-1 gap-4 sm:grid-cols-2 lg:grid-cols-3">
          {links.map((link) => (
            <a
              key={link.label}
              href={link.href}
              target="_blank"
              rel="noopener noreferrer"
              className="flex items-center space-x-3 rounded-lg border border-zinc-200 p-4 
                transition-colors hover:bg-zinc-100 dark:border-zinc-800 
                dark:hover:bg-zinc-800"
            >
              <link.icon />
              <span className="font-medium">{link.username}</span>
            </a>
          ))}
        </div>
      </div>
    </div>
  );
}
