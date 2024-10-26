'use client';
import localFont from 'next/font/local';
import { ThemeProvider } from '@/components/ui/theme-provider';
import './globals.css';
import AuthGaurd from '@/containers/auth-gaurd';
import { Toaster } from '@/components/ui/toaster';

const geistSans = localFont({
  src: './fonts/GeistVF.woff',
  variable: '--font-geist-sans',
  weight: '100 900',
});
const geistMono = localFont({
  src: './fonts/GeistMonoVF.woff',
  variable: '--font-geist-mono',
  weight: '100 900',
});

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html
      lang="en"
      className="dark"
      style={{ colorScheme: 'dark', height: '100%' }}
    >
      <body
        className={`${geistSans.variable} ${geistMono.variable} antialiased`}
        style={{ minHeight: '100%' }}
      >
        <ThemeProvider attribute="class" defaultTheme="dark">
          <AuthGaurd>{children}</AuthGaurd>
          <Toaster />
        </ThemeProvider>
      </body>
    </html>
  );
}
