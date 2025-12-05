import { UnauthorizedException } from '@nestjs/common';
import { Request } from 'express';

interface AuthenticatedUser {
  id: string;
  email: string;
  role: string;
  firstName: string;
  lastName: string;
  username: string;
}

/**
 * Safely gets the authenticated user from the request
 * Throws UnauthorizedException if the user is not authenticated
 */
export function getAuthUser(req: Request): AuthenticatedUser {
  const user = req.user as AuthenticatedUser | undefined;

  if (!user) {
    throw new UnauthorizedException('User is not authenticated');
  }

  return {
    id: user.id,
    email: user.email,
    role: user.role,
    firstName: user.firstName,
    lastName: user.lastName,
    username: user.username,
  };
}
