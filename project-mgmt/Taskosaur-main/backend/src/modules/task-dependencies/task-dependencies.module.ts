import { Module } from '@nestjs/common';
import { TaskDependenciesService } from './task-dependencies.service';
import { TaskDependenciesController } from './task-dependencies.controller';
import { PrismaModule } from '../../prisma/prisma.module';

@Module({
  imports: [PrismaModule],
  controllers: [TaskDependenciesController],
  providers: [TaskDependenciesService],
  exports: [TaskDependenciesService],
})
export class TaskDependenciesModule {}
