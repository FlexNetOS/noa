import { IsUUID, IsNotEmpty } from 'class-validator';

export class UpdateTaskLabelDto {
  @IsUUID()
  @IsNotEmpty()
  userId: string;
}
