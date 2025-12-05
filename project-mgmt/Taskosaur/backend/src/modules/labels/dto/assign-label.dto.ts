import { IsString, IsNotEmpty, IsArray } from 'class-validator';

export class AssignLabelDto {
  @IsString()
  @IsNotEmpty()
  taskId: string;

  @IsString()
  @IsNotEmpty()
  labelId: string;
}

export class AssignMultipleLabelsDto {
  @IsString()
  @IsNotEmpty()
  taskId: string;

  @IsArray()
  @IsString({ each: true })
  labelIds: string[];
}
