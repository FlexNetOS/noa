// src/activity-log/activity-log.module.ts
import { Module } from '@nestjs/common';
import { ActivityLogService } from './activity-log.service';
import { PrismaModule } from 'src/prisma/prisma.module';
import { ActivityLogController } from './activity-log.controller';

@Module({
  imports: [PrismaModule],
  controllers: [ActivityLogController],
  providers: [ActivityLogService],
  exports: [ActivityLogService],
})
export class ActivityLogModule {}
