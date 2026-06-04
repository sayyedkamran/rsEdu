import NextAuth from 'next-auth';
import Google from 'next-auth/providers/google';

const { handlers, signIn, signOut, auth } = NextAuth({
  providers: [
    Google({
      clientId: process.env.GOOGLE_CLIENT_ID!,
      clientSecret: process.env.GOOGLE_CLIENT_SECRET!,
    }),
  ],
  callbacks: {
    async signIn({ user, account }) {
      if (account?.provider === 'google') {
        try {
          const response = await fetch(
            `${process.env.NEXT_PUBLIC_API_URL}/api/v1/auth/google`,
            {
              method: 'POST',
              headers: { 'Content-Type': 'application/json' },
              body: JSON.stringify({
                google_id: account.providerAccountId,
                email: user.email,
                name: user.name,
                picture: user.image,
                access_token: account.access_token,
              }),
            }
          );

          if (response.ok) {
            const data = await response.json();
            (user as any).jwt_token = data.token;
            (user as any).role = data.role;
            (user as any).role_title = data.role_title;
            (user as any).organization_id = data.organization_id;
            (user as any).branch_id = data.branch_id;
            return true;
          }
          return false;
        } catch (error) {
          console.error('Google auth error:', error);
          return false;
        }
      }
      return true;
    },

    async jwt({ token, user }) {
      if (user) {
        token.jwt_token = (user as any).jwt_token;
        token.role = (user as any).role;
        token.role_title = (user as any).role_title;
        token.organization_id = (user as any).organization_id;
        token.branch_id = (user as any).branch_id;
      }
      return token;
    },

    async session({ session, token }) {
      (session as any).jwt_token = token.jwt_token;
      (session as any).role = token.role;
      (session as any).role_title = token.role_title;
      (session as any).organization_id = token.organization_id;
      (session as any).branch_id = token.branch_id;
      return session;
    },
  },
  pages: {
    signIn: '/login',
  },
});

export const { GET, POST } = handlers;