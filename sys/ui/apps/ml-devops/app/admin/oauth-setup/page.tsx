'use client';

import { useState, useEffect } from 'react';
import { useSession } from 'next-auth/react';
import { useRouter } from 'next/navigation';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { Alert, AlertDescription } from '@/components/ui/alert';
import { Loader2, CheckCircle, AlertCircle, Settings, ExternalLink } from 'lucide-react';
import { Separator } from '@/components/ui/separator';

export default function OAuthSetupPage ()
{
  const { data: session, status } = useSession();
  const router = useRouter();
  const [ loading, setLoading ] = useState( false );
  const [ checking, setChecking ] = useState( true );
  const [ configsured, setconfigsured ] = useState( false );
  const [ error, setError ] = useState( '' );
  const [ success, setSuccess ] = useState( '' );
  const [ clientId, setClientId ] = useState( '' );
  const [ clientSecret, setClientSecret ] = useState( '' );

  useEffect( () =>
  {
    if ( status === 'unauthenticated' )
    {
      router.push( '/login' );
    } else if ( status === 'authenticated' )
    {
      checkconfigsuration();
    }
  }, [ status, router ] );

  const checkconfigsuration = async () =>
  {
    try
    {
      const res = await fetch( '/api/admin/oauth-configs' );
      const data = await res.json();
      setconfigsured( data.configsured );
      if ( data.clientId )
      {
        setClientId( data.clientId );
      }
    } catch ( err )
    {
      console.error( 'Failed to check OAuth configsuration:', err );
    } finally
    {
      setChecking( false );
    }
  };

  const handleSave = async ( e: React.FormEvent ) =>
  {
    e.preventDefault();
    setError( '' );
    setSuccess( '' );
    setLoading( true );

    try
    {
      const res = await fetch( '/api/admin/oauth-configs', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify( { clientId, clientSecret } ),
      } );

      const data = await res.json();

      if ( !res.ok )
      {
        throw new Error( data.error || 'Failed to save configsuration' );
      }

      setSuccess( 'OAuth configsuration saved successfully! Users can now sign in with Google.' );
      setconfigsured( true );
    } catch ( err: any )
    {
      setError( err.message || 'An error occurred' );
    } finally
    {
      setLoading( false );
    }
  };

  const handleTest = async () =>
  {
    setError( '' );
    setSuccess( '' );
    setLoading( true );

    try
    {
      const res = await fetch( '/api/admin/oauth-configs/test' );
      const data = await res.json();

      if ( !res.ok )
      {
        throw new Error( data.error || 'Test failed' );
      }

      setSuccess( 'OAuth configsuration is valid! Google SSO is working correctly.' );
    } catch ( err: any )
    {
      setError( err.message || 'Test failed' );
    } finally
    {
      setLoading( false );
    }
  };

  if ( status === 'loading' || checking )
  {
    return (
      <div className="min-h-screen flex items-center justify-center">
        <Loader2 className="h-8 w-8 animate-spin text-primary" />
      </div>
    );
  }

  return (
    <div className="min-h-screen bg-gradient-to-br from-background via-background to-muted p-4 md:p-8">
      <div className="max-w-4xl mx-auto space-y-6">
        <div className="flex items-center gap-4">
          <Button variant="outline" onClick={ () => router.push( '/' ) }>
            ← Back
          </Button>
          <div>
            <h1 className="text-3xl font-bold">OAuth Setup</h1>
            <p className="text-muted-foreground">configsure Google Single Sign-On</p>
          </div>
        </div>

        { configsured && (
          <Alert className="border-green-500 bg-green-50 dark:bg-green-900/20">
            <CheckCircle className="h-4 w-4 text-green-600" />
            <AlertDescription className="text-green-800 dark:text-green-200">
              Google SSO is configsured and active. Users can sign in with their Google accounts.
            </AlertDescription>
          </Alert>
        ) }

        <Card>
          <CardHeader>
            <CardTitle className="flex items-center gap-2">
              <Settings className="h-5 w-5" />
              Google OAuth configsuration
            </CardTitle>
            <CardDescription>
              Set up Google OAuth credentials to enable "Sign in with Google" for all users.
            </CardDescription>
          </CardHeader>
          <CardContent className="space-y-6">
            <div className="p-4 border rounded-lg bg-muted/50 space-y-3">
              <h3 className="font-semibold flex items-center gap-2">
                <span className="text-lg">📋</span>
                Setup Instructions
              </h3>
              <ol className="space-y-2 text-sm text-muted-foreground pl-6 list-decimal">
                <li>
                  Go to{ ' ' }
                  <a
                    href="https://console.cloud.google.com"
                    target="_blank"
                    rel="noopener noreferrer"
                    className="text-primary hover:underline inline-flex items-center gap-1"
                  >
                    Google Cloud Console
                    <ExternalLink className="h-3 w-3" />
                  </a>
                </li>
                <li>Create a new project or select an existing one</li>
                <li>Navigate to &quot;APIs & Services&quot; → &quot;Credentials&quot;</li>
                <li>Click &quot;Create Credentials&quot; → &quot;OAuth 2.0 Client ID&quot;</li>
                <li>configsure the OAuth consent screen if prompted</li>
                <li>Select &quot;Web application&quot; as the application type</li>
                <li>
                  Add authorized redirect URIs:
                  <ul className="list-disc pl-6 mt-1 space-y-1">
                    <li className="font-mono text-xs">http://localhost:3000/api/auth/callback/google</li>
                    <li className="font-mono text-xs">https://de-flex.net/api/auth/callback/google</li>
                  </ul>
                </li>
                <li>Copy the Client ID and Client Secret</li>
                <li>Paste them into the form below and save</li>
              </ol>
            </div>

            <Separator />

            <form onSubmit={ handleSave } className="space-y-4">
              { error && (
                <Alert variant="destructive">
                  <AlertCircle className="h-4 w-4" />
                  <AlertDescription>{ error }</AlertDescription>
                </Alert>
              ) }

              { success && (
                <Alert className="border-green-500 bg-green-50 dark:bg-green-900/20">
                  <CheckCircle className="h-4 w-4 text-green-600" />
                  <AlertDescription className="text-green-800 dark:text-green-200">
                    { success }
                  </AlertDescription>
                </Alert>
              ) }

              <div className="space-y-2">
                <Label htmlFor="clientId">Google Client ID</Label>
                <Input
                  id="clientId"
                  type="text"
                  placeholder="1234567890-abc123xyz.apps.googleusercontent.com"
                  value={ clientId }
                  onChange={ ( e ) => setClientId( e.target.value ) }
                  required
                />
                <p className="text-xs text-muted-foreground">
                  This is safe to expose publicly and will be visible in your HTML source.
                </p>
              </div>

              <div className="space-y-2">
                <Label htmlFor="clientSecret">Google Client Secret</Label>
                <Input
                  id="clientSecret"
                  type="password"
                  placeholder="GOCSPX-abc123xyz..."
                  value={ clientSecret }
                  onChange={ ( e ) => setClientSecret( e.target.value ) }
                  required={ !configsured }
                />
                <p className="text-xs text-muted-foreground">
                  Keep this secret secure. It will be encrypted and stored safely.
                </p>
              </div>

              <div className="flex gap-3">
                <Button type="submit" disabled={ loading }>
                  { loading ? (
                    <>
                      <Loader2 className="h-4 w-4 animate-spin mr-2" />
                      Saving...
                    </>
                  ) : configsured ? (
                    'Update configsuration'
                  ) : (
                    'Save configsuration'
                  ) }
                </Button>

                { configsured && (
                  <Button type="button" variant="outline" onClick={ handleTest } disabled={ loading }>
                    Test configsuration
                  </Button>
                ) }
              </div>
            </form>
          </CardContent>
        </Card>

        <Card>
          <CardHeader>
            <CardTitle>Security Notes</CardTitle>
          </CardHeader>
          <CardContent className="space-y-2 text-sm text-muted-foreground">
            <p>• OAuth credentials are stored encrypted in the database</p>
            <p>• Only administrators can view or modify these settings</p>
            <p>• Client Secret is never exposed to the client-side code</p>
            <p>• All OAuth flows use secure HTTPS connections in production</p>
            <p>• Users&apos; Google account data is only used for authentication</p>
          </CardContent>
        </Card>
      </div>
    </div>
  );
}
