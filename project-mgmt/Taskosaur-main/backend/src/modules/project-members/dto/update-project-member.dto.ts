import { IsEnum, IsOptional } from 'class-validator';
import { Role as ProjectRole } from '@prisma/client';

export class UpdateProjectMemberDto {
  @IsEnum(ProjectRole)
  @IsOptional()
  role?: ProjectRole;
}
