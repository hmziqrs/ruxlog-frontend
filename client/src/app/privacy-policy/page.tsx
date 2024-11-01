import { Metadata } from 'next';

export const metadata: Metadata = {
  title: 'Privacy Policy | [Your Site Name]',
  description: 'Learn how [Your Site Name] collects and uses your data.',
};

export default function PrivacyPolicyPage() {
  return (
    <main className="container mx-auto py-8 px-5">
      <h1 className="text-3xl font-bold mb-4 text-zinc-900 dark:text-zinc-100">
        Privacy Policy
      </h1>

      <p className="text-zinc-600 dark:text-zinc-400 mb-6">
        This Privacy Policy describes how [Your Site Name] ("we", "us", or
        "our") collects, uses, discloses, and protects your personal information
        when you visit and interact with our website [Your Site URL] (the
        "Site").
      </p>

      <h2 className="text-2xl font-bold mb-4 text-zinc-900 dark:text-zinc-100">
        1. Information We Collect
      </h2>

      <p className="text-zinc-600 dark:text-zinc-400 mb-4">
        We collect the following information when you use the Site:
      </p>

      <ul className="list-disc pl-6 text-zinc-600 dark:text-zinc-400 mb-4">
        <li>
          <strong>Personal Information:</strong>
          <ul className="list-disc pl-6 text-zinc-600 dark:text-zinc-400 mb-4">
            <li>
              <strong>Account Information:</strong> If you create an account, we
              collect information such as your name, email address, and
              password.
            </li>
            <li>
              <strong>Comments and Feedback:</strong> If you leave a comment or
              contact us, we collect your name, email address, and any other
              information you provide.
            </li>
            <li>
              <strong>Social Media:</strong> If you choose to interact with our
              website using social media accounts, we may collect information
              such as your username, profile picture, and other publicly
              available information.
            </li>
          </ul>
        </li>
        <li>
          <strong>Usage Information:</strong>
          <ul className="list-disc pl-6 text-zinc-600 dark:text-zinc-400 mb-4">
            <li>
              <strong>Log Data:</strong> We collect information about your
              device, browser, operating system, and other technical information
              when you visit the Site, such as your IP address, browsing
              activity, and referring URLs.
            </li>
            <li>
              <strong>Cookies and Similar Technologies:</strong> We use cookies
              and similar technologies to collect information about your
              browsing activity, preferences, and device. This information helps
              us improve the Site and deliver personalized content.
            </li>
          </ul>
        </li>
      </ul>

      <h2 className="text-2xl font-bold mb-4 text-zinc-900 dark:text-zinc-100">
        2. How We Use Your Information
      </h2>

      <p className="text-zinc-600 dark:text-zinc-400 mb-4">
        We use your information for the following purposes:
      </p>

      <ul className="list-disc pl-6 text-zinc-600 dark:text-zinc-400 mb-4">
        <li>
          <strong>To provide and improve the Site:</strong> We use your
          information to operate and improve the functionality of the Site,
          including to provide content, process comments, and manage user
          accounts.
        </li>
        <li>
          <strong>To communicate with you:</strong> We use your information to
          respond to your inquiries, send you updates about the Site, and
          provide customer support.
        </li>
        <li>
          <strong>To personalize your experience:</strong> We use your
          information to personalize your experience on the Site, including by
          delivering tailored content and recommendations.
        </li>
        <li>
          <strong>For marketing and advertising:</strong> We may use your
          information to send you marketing emails or other promotional
          materials about our services.
        </li>
        <li>
          <strong>To analyze and understand user behavior:</strong> We use your
          information to analyze and understand how users interact with the
          Site, which helps us improve its functionality and content.
        </li>
        <li>
          <strong>To comply with legal obligations:</strong> We may use your
          information to comply with applicable laws, regulations, and legal
          requests.
        </li>
      </ul>

      <h2 className="text-2xl font-bold mb-4 text-zinc-900 dark:text-zinc-100">
        3. Sharing Your Information
      </h2>

      <p className="text-zinc-600 dark:text-zinc-400 mb-4">
        We may share your information with:
      </p>

      <ul className="list-disc pl-6 text-zinc-600 dark:text-zinc-400 mb-4">
        <li>
          <strong>Third-party service providers:</strong> We may use third-party
          service providers to assist us with providing and improving the Site,
          including data analysis, marketing, and customer support.
        </li>
        <li>
          <strong>Social media platforms:</strong> If you choose to connect with
          our website using social media, we may share your information with
          those platforms.
        </li>
        <li>
          <strong>Legal authorities:</strong> We may disclose your information
          to law enforcement agencies or other third parties if we believe it is
          necessary to comply with legal obligations, protect the rights of
          others, or prevent harm.
        </li>
      </ul>

      <h2 className="text-2xl font-bold mb-4 text-zinc-900 dark:text-zinc-100">
        4. Security
      </h2>

      <p className="text-zinc-600 dark:text-zinc-400 mb-4">
        We take reasonable measures to protect your personal information from
        unauthorized access, use, or disclosure. However, no website or internet
        transmission is completely secure. Therefore, we cannot guarantee the
        absolute security of your information.
      </p>

      <h2 className="text-2xl font-bold mb-4 text-zinc-900 dark:text-zinc-100">
        5. Your Choices
      </h2>

      <p className="text-zinc-600 dark:text-zinc-400 mb-4">
        You have the following choices regarding your personal information:
      </p>

      <ul className="list-disc pl-6 text-zinc-600 dark:text-zinc-400 mb-4">
        <li>
          <strong>Opt out of marketing emails:</strong> You can opt out of
          receiving marketing emails from us by following the unsubscribe
          instructions provided in the email.
        </li>
        <li>
          <strong>Control cookies:</strong> You can control the cookies used on
          our website by adjusting your browser settings.
        </li>
        <li>
          <strong>Delete your account:</strong> You can delete your account by
          contacting us.
        </li>
      </ul>

      <h2 className="text-2xl font-bold mb-4 text-zinc-900 dark:text-zinc-100">
        6. Children's Privacy
      </h2>

      <p className="text-zinc-600 dark:text-zinc-400 mb-4">
        The Site is not intended for children under the age of 13. We do not
        knowingly collect personal information from children under 13. If you
        are a parent or guardian and you believe that your child has provided us
        with personal information, please contact us.
      </p>

      <h2 className="text-2xl font-bold mb-4 text-zinc-900 dark:text-zinc-100">
        7. Updates to this Privacy Policy
      </h2>

      <p className="text-zinc-600 dark:text-zinc-400 mb-4">
        We may update this Privacy Policy from time to time. We will post any
        changes on the Site. We encourage you to review this Privacy Policy
        periodically.
      </p>

      <h2 className="text-2xl font-bold mb-4 text-zinc-900 dark:text-zinc-100">
        8. Contact Us
      </h2>

      <p className="text-zinc-600 dark:text-zinc-400 mb-4">
        If you have any questions about this Privacy Policy, please contact us
        at [Your Email Address].
      </p>

      <p className="text-zinc-600 dark:text-zinc-400 mb-4">
        Last Updated: [Date]
      </p>
    </main>
  );
}
