import { PartialType } from '@nestjs/mapped-types';
import { CreateTaskStatusDto } from './create-task-status.dto';
import { IsArray, IsNumber, IsString, ValidateNested } from 'class-validator';
import { Type } from 'class-transformer';

export class UpdateTaskStatusDto extends PartialType(CreateTaskStatusDto) {}
export class UpdatePositionItemDto {
  @IsString()
  id: string;

  @IsNumber()
  position: number;
}

export class UpdatePositionsDto {
  @IsArray()
  @ValidateNested({ each: true })
  @Type(() => UpdatePositionItemDto)
  statusUpdates: UpdatePositionItemDto[];
}
