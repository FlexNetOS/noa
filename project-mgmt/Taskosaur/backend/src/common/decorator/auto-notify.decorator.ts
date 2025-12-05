// src/common/decorators/auto-notify.decorator.ts
import { SetMetadata } from '@nestjs/common';
import { NotificationType, NotificationPriority } from '@prisma/client';

export interface NotificationConfig {
  type: NotificationType;
  title?: string;
  message?: string;
  entityType?: string;
  entityId?: string;
  actionUrl?: string;
  priority?: NotificationPriority;
  notifyUserId?: string;
  condition?: (requestData: any, responseData: any) => boolean;
}

export const AutoNotify = (config: NotificationConfig) =>
  SetMetadata('notification-config', config);
