import { NextRequest, NextResponse } from 'next/server';
import { getServerSession } from 'next-auth';
import { authOptions } from '@/lib/auth';
import { prisma } from '@/lib/db';
import crypto from 'crypto';

export const dynamic = 'force-dynamic';

// Encryption key from environment (fallback to a default for development)
const ENCRYPTION_KEY = process.env.OAUTH_ENCRYPTION_KEY || 'default-32-character-encryption-key-change-me';
const ALGORITHM = 'aes-256-cbc';

function encrypt ( text: string ): string
{
  const iv = crypto.randomBytes( 16 );
  const key = Buffer.from( ENCRYPTION_KEY.padEnd( 32, '0' ).slice( 0, 32 ) );
  const cipher = crypto.createCipheriv( ALGORITHM, key, iv );
  let encrypted = cipher.update( text, 'utf8', 'hex' );
  encrypted += cipher.final( 'hex' );
  return iv.toString( 'hex' ) + ':' + encrypted;
}

function decrypt ( text: string ): string
{
  const parts = text.split( ':' );
  const iv = Buffer.from( parts[ 0 ], 'hex' );
  const encryptedText = parts[ 1 ];
  const key = Buffer.from( ENCRYPTION_KEY.padEnd( 32, '0' ).slice( 0, 32 ) );
  const decipher = crypto.createDecipheriv( ALGORITHM, key, iv );
  let decrypted = decipher.update( encryptedText, 'hex', 'utf8' );
  decrypted += decipher.final( 'utf8' );
  return decrypted;
}

export async function GET ( req: NextRequest )
{
  try
  {
    const session = await getServerSession( authOptions );

    if ( !session?.user )
    {
      return NextResponse.json( { error: 'Unauthorized' }, { status: 401 } );
    }

    // Check if user is admin (you can modify this check based on your requirements)
    const user = await prisma.user.findUnique( {
      where: { email: session.user.email! },
    } );

    if ( user?.role !== 'admin' )
    {
      return NextResponse.json( { error: 'Forbidden - Admin access required' }, { status: 403 } );
    }

    // Get OAuth configs for Google
    const configs = await prisma.oAuthconfigs.findUnique( {
      where: { provider: 'google' },
    } );

    if ( !configs )
    {
      return NextResponse.json( { configsured: false } );
    }

    return NextResponse.json( {
      configsured: true,
      clientId: configs.clientId,
      enabled: configs.enabled,
    } );
  } catch ( error )
  {
    console.error( 'Error fetching OAuth configs:', error );
    return NextResponse.json(
      { error: 'Internal server error' },
      { status: 500 }
    );
  }
}

export async function POST ( req: NextRequest )
{
  try
  {
    const session = await getServerSession( authOptions );

    if ( !session?.user )
    {
      return NextResponse.json( { error: 'Unauthorized' }, { status: 401 } );
    }

    // Check if user is admin
    const user = await prisma.user.findUnique( {
      where: { email: session.user.email! },
    } );

    if ( user?.role !== 'admin' )
    {
      return NextResponse.json( { error: 'Forbidden - Admin access required' }, { status: 403 } );
    }

    const { clientId, clientSecret } = await req.json();

    if ( !clientId )
    {
      return NextResponse.json( { error: 'Client ID is required' }, { status: 400 } );
    }

    // Check if configs exists
    const existing = await prisma.oAuthconfigs.findUnique( {
      where: { provider: 'google' },
    } );

    let encryptedSecret = existing?.clientSecret;

    // Only update secret if provided
    if ( clientSecret )
    {
      encryptedSecret = encrypt( clientSecret );
    } else if ( !existing )
    {
      return NextResponse.json( { error: 'Client Secret is required for initial setup' }, { status: 400 } );
    }

    // Upsert OAuth configs
    await prisma.oAuthconfigs.upsert( {
      where: { provider: 'google' },
      update: {
        clientId,
        ...( clientSecret && { clientSecret: encryptedSecret } ),
        updatedAt: new Date(),
      },
      create: {
        provider: 'google',
        clientId,
        clientSecret: encryptedSecret!,
        enabled: true,
      },
    } );

    return NextResponse.json( { success: true, message: 'OAuth configsuration saved successfully' } );
  } catch ( error )
  {
    console.error( 'Error saving OAuth configs:', error );
    return NextResponse.json(
      { error: 'Internal server error' },
      { status: 500 }
    );
  }
}
