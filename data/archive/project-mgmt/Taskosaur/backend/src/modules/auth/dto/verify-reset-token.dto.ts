import { ApiProperty } from '@nestjs/swagger';

export class VerifyResetTokenResponseDto {
  @ApiProperty({
    description: 'Whether the token is valid',
    example: true,
  })
  valid: boolean;

  @ApiProperty({
    description: 'Additional message about token validity',
    example: 'Token is valid',
    required: false,
  })
  message?: string;
}
