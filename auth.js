<<<<<<< HEAD:auth.js
import { NextAuthOptions } from 'next-auth';
import GithubProvider from 'next-auth/providers/github';
import CredentialProvider from 'next-auth/providers/credentials';
=======
import { NextAuthConfig } from 'next-auth';
import CredentialProvider from 'next-auth/providers/credentials';
import GithubProvider from 'next-auth/providers/github';
>>>>>>> cb3da2f (code formatted):auth.config.ts

export const { auth, handlers, signIn, signOut } = NextAuth({
  providers: [
    GithubProvider({
      clientId: process.env.GITHUB_ID ?? '',
      clientSecret: process.env.GITHUB_SECRET ?? ''
    }),
    CredentialProvider({
      credentials: {
        email: {
<<<<<<< HEAD:auth.js
          label: 'email',
          type: 'email',
          placeholder: 'example@gmail.com'
        }
      },
      async authorize(credentials, req) {
        const user = { id: '1', name: 'John', email: credentials?.email };
=======
          type: 'email'
        },
        password: {
          type: 'password'
        }
      },
      async authorize(credentials, req) {
        const user = {
          id: '1',
          name: 'John',
          email: credentials?.email as string
        };
>>>>>>> cb3da2f (code formatted):auth.config.ts
        if (user) {
          // Any object returned will be saved in `user` property of the JWT
          return user;
        } else {
          // If you return null then an error will be displayed advising the user to check their details.
          return null;

          // You can also Reject this callback with an Error thus the user will be sent to the error page with the error message as a query parameter
        }
      }
    })
  ],
  pages: {
    signIn: '/' //sigin page
  }
<<<<<<< HEAD:auth.js
});
=======
} satisfies NextAuthConfig;

export default authConfig;
>>>>>>> cb3da2f (code formatted):auth.config.ts