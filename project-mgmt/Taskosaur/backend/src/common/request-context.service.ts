import { Injectable } from '@nestjs/common';
import { AsyncLocalStorage } from 'async_hooks';

export interface RequestContext {
  userId?: string;
  user?: {
    id: string;
    email: string;
    firstName: string;
    lastName: string;
    role: string;
  };
}

@Injectable()
export class RequestContextService {
  private static asyncLocalStorage = new AsyncLocalStorage<RequestContext>();

  static run<T>(context: RequestContext, callback: () => T): T {
    return this.asyncLocalStorage.run(context, callback);
  }

  static getCurrentContext(): RequestContext | undefined {
    return this.asyncLocalStorage.getStore();
  }

  static getCurrentUserId(): string | undefined {
    const context = this.getCurrentContext();
    return context?.userId || context?.user?.id;
  }

  static getCurrentUser(): RequestContext['user'] | undefined {
    const context = this.getCurrentContext();
    return context?.user;
  }

  static setUserId(userId: string): void {
    const context = this.getCurrentContext();
    if (context) {
      context.userId = userId;
    }
  }

  static setUser(user: RequestContext['user']): void {
    const context = this.getCurrentContext();
    if (context && user) {
      context.user = user;
      context.userId = user.id;
    }
  }
}
