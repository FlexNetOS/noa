import { IsString, IsNotEmpty } from 'class-validator';

export class UpdateTaskCommentDto {
  @IsString()
  @IsNotEmpty()
  content: string;
}
