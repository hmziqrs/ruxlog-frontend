import Providers from '@/components/layout/providers';
import { Toaster } from '@/components/ui/sonner';
import type { Metadata } from 'next';
import { Lato } from 'next/font/google';
import NextTopLoader from 'nextjs-toploader';
import './globals.css';
<<<<<<< HEAD
import { getServerSession } from 'next-auth';

const inter = Inter({ subsets: ['latin'] });
=======
import AuthGaurd from '@/components/layout/auth-gaurd';
>>>>>>> 7d11553 (cookie delete fix)

export const metadata: Metadata = {
  title: 'Next Shadcn',
  description: 'Basic dashboard with Next.js and Shadcn'
};

const lato = Lato({
  subsets: ['latin'],
  weight: ['400', '700', '900'],
  display: 'swap'
});

export default function RootLayout({
  children
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="en" className={`${lato.className}`}>
      <body className={'overflow-hidden'} suppressHydrationWarning={true}>
        <NextTopLoader showSpinner={false} />
        <Providers>
          <AuthGaurd>
            <Toaster />
            {children}
          </AuthGaurd>
        </Providers>
      </body>
    </html>
  );
}
