import { IsString, IsNotEmpty, IsEnum, IsOptional } from 'class-validator';
import { Role as ProjectRole } from '@prisma/client';

export class CreateProjectMemberDto {
  @IsString()
  @IsNotEmpty()
  userId: string;

  @IsString()
  @IsNotEmpty()
  projectId: string;

  @IsEnum(ProjectRole)
  @IsOptional()
  role?: ProjectRole;
}

export class InviteProjectMemberDto {
  @IsString()
  @IsNotEmpty()
  email: string;

  @IsString()
  @IsNotEmpty()
  projectId: string;

  @IsEnum(ProjectRole)
  @IsOptional()
  role?: ProjectRole;
}
