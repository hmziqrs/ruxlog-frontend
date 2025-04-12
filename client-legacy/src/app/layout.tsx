import type { Metadata } from 'next';
import localFont from 'next/font/local';
import { ScreenTracker } from '@/components/screen-tracker';
import { LayoutFooter } from '@/components/layout-footer';

import './globals.css';
import { LayoutHeader } from '@/components/layout-header';
import { Suspense } from 'react';

const geistSans = localFont({
  src: './fonts/GeistVF.woff',
  variable: '--font-geist-sans',
  weight: '100 400 600 900',
});
const geistMono = localFont({
  src: './fonts/GeistMonoVF.woff',
  variable: '--font-geist-mono',
  weight: '100 400 600 900',
});

export const metadata: Metadata = {
  title: 'hmziq | Personal blog',
  description: 'Tech and personal blog of hmziq',
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  console.log('We testing blog');
  console.log(process.env);
  return (
    <>
      <Suspense>
        <ScreenTracker />
      </Suspense>
      <html lang="en" className="dark h-full">
        <head>
          <link
            rel="icon"
            type="image/png"
            href="/fav/favicon-96x96.png"
            sizes="96x96"
          />
          <link rel="icon" type="image/svg+xml" href="/fav/favicon.svg" />
          <link rel="shortcut icon" href="/fav/favicon.ico" />
          <link
            rel="apple-touch-icon"
            sizes="180x180"
            href="/fav/apple-touch-icon.png"
          />
          <meta name="mobile-web-app-capable" content="yes" />
          <link rel="manifest" href="/fav/site.webmanifest" />
        </head>
        <body
          className={`${geistSans.variable} ${geistMono.variable} antialiased min-h-full flex flex-col dark:text-white text-black dark:bg-zinc-950 bg-white transition-colors duration-500`}
        >
          <LayoutHeader />
          <div className="flex flex-grow container mx-auto">{children}</div>
          <LayoutFooter />
        </body>
      </html>
    </>
  );
}
