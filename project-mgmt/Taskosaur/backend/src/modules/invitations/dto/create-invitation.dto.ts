// src/modules/invitations/dto/create-invitation.dto.ts
import { IsEmail, IsString, IsOptional, IsUUID } from 'class-validator';

export class CreateInvitationDto {
  @IsEmail()
  inviteeEmail: string;

  @IsOptional()
  @IsUUID()
  organizationId?: string;

  @IsOptional()
  @IsUUID()
  workspaceId?: string;

  @IsOptional()
  @IsUUID()
  projectId?: string;

  @IsString()
  role: string;
}
