import { Module } from '@nestjs/common';
import { TaskWatchersService } from './task-watchers.service';
import { TaskWatchersController } from './task-watchers.controller';
import { PrismaModule } from '../../prisma/prisma.module';

@Module({
  imports: [PrismaModule],
  controllers: [TaskWatchersController],
  providers: [TaskWatchersService],
  exports: [TaskWatchersService],
})
export class TaskWatchersModule {}
