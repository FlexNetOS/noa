import { withAuth } from 'next-auth/middleware';
import { NextResponse } from 'next/server';

export default withAuth(
  function middleware(req) {
    return NextResponse.next();
  },
  {
    callbacks: {
      authorized: ({ token, req }) => {
        // Public routes - no auth required
        const publicPaths = ['/login', '/signup', '/api/auth', '/api/signup'];
        const isPublicPath = publicPaths.some(path => 
          req.nextUrl.pathname.startsWith(path)
        );
        
        if (isPublicPath) return true;
        
        // All other routes require authentication
        return !!token;
      },
    },
  }
);

export const config = {
  matcher: [
    '/((?!_next/static|_next/image|favicon.ico|favicon.svg|og-image.png|robots.txt|api/auth).*)',
  ],
};
