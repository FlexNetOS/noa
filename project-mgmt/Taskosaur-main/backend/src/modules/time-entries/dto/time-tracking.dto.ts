import { IsUUID, IsNotEmpty, IsOptional, IsString } from 'class-validator';

export class StartTimerDto {
  @IsUUID()
  @IsNotEmpty()
  taskId: string;

  @IsUUID()
  @IsNotEmpty()
  userId: string;

  @IsString()
  @IsOptional()
  description?: string;
}

export class StopTimerDto {
  @IsUUID()
  @IsNotEmpty()
  userId: string;

  @IsString()
  @IsOptional()
  description?: string;
}
