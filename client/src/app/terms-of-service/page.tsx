import { Metadata } from 'next';

export const metadata: Metadata = {
  title: 'Terms of Service | ' + process.env.NEXT_PUBLIC_SITE_NAME,
  description: `Read the terms and conditions for using ${process.env.NEXT_PUBLIC_SITE_NAME}.`,
};

export default function TermsOfServicePage() {
  return (
    <main className="container mx-auto py-8 px-5">
      <h1 className="text-3xl font-bold mb-4 text-zinc-900 dark:text-zinc-100">
        Terms of Service
      </h1>

      <p className="text-zinc-600 dark:text-zinc-400 mb-6">
        Welcome to {process.env.NEXT_PUBLIC_SITE_NAME}! These Terms of Service
        (&quot;Terms&quot;, &quot;Agreement&quot;) govern your access to and use
        of the website {process.env.NEXT_PUBLIC_SITE_URL} and its related
        services (collectively, the &quot;Site&quot;), operated by{' '}
        {process.env.NEXT_PUBLIC_SITE_NAME}.
      </p>

      <h2 className="text-2xl font-bold mb-4 text-zinc-900 dark:text-zinc-100">
        1. Acceptance of Terms
      </h2>

      <p className="text-zinc-600 dark:text-zinc-400 mb-4">
        By accessing or using the Site, you agree to be bound by these Terms. If
        you disagree with any part of the Terms, you may not access or use the
        Site.
      </p>

      <h2 className="text-2xl font-bold mb-4 text-zinc-900 dark:text-zinc-100">
        2. Modifications to Terms
      </h2>

      <p className="text-zinc-600 dark:text-zinc-400 mb-4">
        We may modify these Terms at any time without prior notice. Your
        continued use of the Site following any changes constitutes your
        acceptance of the revised Terms. We encourage you to periodically review
        the Terms for any updates.
      </p>

      <h2 className="text-2xl font-bold mb-4 text-zinc-900 dark:text-zinc-100">
        3. User Conduct
      </h2>

      <p className="text-zinc-600 dark:text-zinc-400 mb-4">You agree not to:</p>

      <ul className="list-disc pl-6 text-zinc-600 dark:text-zinc-400 mb-4">
        <li>Use the Site for any unlawful or unauthorized purpose.</li>
        <li>
          Violate any local, state, national, or international law or
          regulation.
        </li>
        <li>
          Infringe upon the intellectual property rights of others, including
          copyrights, trademarks, patents, or trade secrets.
        </li>
        <li>
          Upload or transmit any content that is illegal, harmful, threatening,
          abusive, harassing, defamatory, obscene, or otherwise objectionable.
        </li>
        <li>
          Interfere with or disrupt the Site or servers connected to the Site.
        </li>
        <li>
          Attempt to gain unauthorized access to the Site, other accounts,
          computer systems, or networks connected to the Site.
        </li>
        <li>
          Impersonate any person or entity or falsely state or misrepresent your
          affiliation with any person or entity.
        </li>
        <li>Upload or transmit any viruses, malware, or other harmful code.</li>
        <li>
          Collect or store personal information about others without their
          consent.
        </li>
        <li>
          Use any automated means, including robots, spiders, or scrapers, to
          access the Site or its content.
        </li>
        <li>
          Engage in any other activity that violates these Terms or that is
          deemed harmful or inappropriate by us.
        </li>
      </ul>

      <h2 className="text-2xl font-bold mb-4 text-zinc-900 dark:text-zinc-100">
        4. Content
      </h2>

      <p className="text-zinc-600 dark:text-zinc-400 mb-4">
        All content on the Site, including but not limited to text, graphics,
        images, logos, audio, video, software, and other materials, is the
        property of {process.env.NEXT_PUBLIC_SITE_NAME} or its licensors and is
        protected by copyright and other intellectual property laws. You may not
        reproduce, distribute, modify, create derivative works of, publicly
        display, or commercially exploit any content from the Site without our
        express written permission.
      </p>

      <h2 className="text-2xl font-bold mb-4 text-zinc-900 dark:text-zinc-100">
        5. User Accounts
      </h2>

      <p className="text-zinc-600 dark:text-zinc-400 mb-4">
        If you create an account on the Site, you are responsible for
        maintaining the confidentiality of your account and password. You are
        also responsible for all activities that occur under your account. You
        agree to notify us immediately of any unauthorized use of your account
        or any other security breach. We are not responsible for any loss or
        damage resulting from your failure to comply with these security
        obligations.
      </p>

      <h2 className="text-2xl font-bold mb-4 text-zinc-900 dark:text-zinc-100">
        6. Termination
      </h2>

      <p className="text-zinc-600 dark:text-zinc-400 mb-4">
        We may terminate your access to the Site at any time, for any reason,
        without prior notice. We may also terminate your access to the Site if
        you violate these Terms.
      </p>

      <h2 className="text-2xl font-bold mb-4 text-zinc-900 dark:text-zinc-100">
        7. Disclaimer
      </h2>

      <p className="text-zinc-600 dark:text-zinc-400 mb-4">
        The Site is provided &quot;as is&quot; without warranty of any kind,
        express or implied. We disclaim all warranties, including but not
        limited to, warranties of merchantability, fitness for a particular
        purpose, and non-infringement. We do not warrant that the Site will be
        uninterrupted or error-free, that defects will be corrected, or that the
        Site or the server that makes it available are free of viruses or other
        harmful components. We do not warrant or make any representations
        regarding the use or the results of the use of the Site in terms of
        accuracy, reliability, or otherwise.
      </p>

      <h2 className="text-2xl font-bold mb-4 text-zinc-900 dark:text-zinc-100">
        8. Limitation of Liability
      </h2>

      <p className="text-zinc-600 dark:text-zinc-400 mb-4">
        To the maximum extent permitted by applicable law, we will not be liable
        for any direct, indirect, incidental, consequential, special, or
        exemplary damages arising from or relating to your access to or use of
        the Site, including but not limited to damages for loss of profits,
        goodwill, use, data, or other intangible losses, even if we have been
        advised of the possibility of such damages.
      </p>

      <h2 className="text-2xl font-bold mb-4 text-zinc-900 dark:text-zinc-100">
        9. Indemnification
      </h2>

      <p className="text-zinc-600 dark:text-zinc-400 mb-4">
        You agree to indemnify and hold harmless{' '}
        {process.env.NEXT_PUBLIC_SITE_NAME}, its officers, directors, employees,
        agents, licensors, and suppliers from and against any and all claims,
        demands, losses, liabilities, costs and expenses (including
        attorney&apos;s fees) arising out of or relating to your use of the
        Site, your violation of these Terms, or your violation of any rights of
        another.
      </p>

      <h2 className="text-2xl font-bold mb-4 text-zinc-900 dark:text-zinc-100">
        10. Entire Agreement
      </h2>

      <p className="text-zinc-600 dark:text-zinc-400 mb-4">
        These Terms constitute the entire agreement between you and{' '}
        {process.env.NEXT_PUBLIC_SITE_NAME} regarding your use of the Site and
        supersede all prior or contemporaneous communications and proposals,
        whether oral or written.
      </p>

      <h2 className="text-2xl font-bold mb-4 text-zinc-900 dark:text-zinc-100">
        11. Contact Us
      </h2>

      <p className="text-zinc-600 dark:text-zinc-400 mb-4">
        If you have any questions about these Terms, please contact us at{' '}
        {process.env.NEXT_PUBLIC_CONTACT_EMAIL}.
      </p>

      <p className="text-zinc-600 dark:text-zinc-400 mb-4">
        Last Updated:{' '}
        {new Date('2024-11-01').toLocaleDateString('en-US', {
          month: 'long',
          day: 'numeric',
          year: 'numeric',
        })}
      </p>
    </main>
  );
}
