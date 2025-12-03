import { Module } from '@nestjs/common';
import { GanttService } from './gantt.service';
import { GanttController } from './gantt.controller';
import { PrismaModule } from '../../prisma/prisma.module';

@Module({
  imports: [PrismaModule],
  controllers: [GanttController],
  providers: [GanttService],
  exports: [GanttService],
})
export class GanttModule {}
