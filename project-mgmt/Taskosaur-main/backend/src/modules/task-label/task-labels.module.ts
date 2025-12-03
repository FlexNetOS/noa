import { Module } from '@nestjs/common';
import { TaskLabelsService } from './task-labels.service';
import { TaskLabelsController } from './task-labels.controller';
import { PrismaModule } from '../../prisma/prisma.module';

@Module({
  imports: [PrismaModule],
  controllers: [TaskLabelsController],
  providers: [TaskLabelsService],
  exports: [TaskLabelsService],
})
export class TaskLabelsModule {}
