import { ProviderClient } from '@/services/providerClient';

const fetchMock = global.fetch as unknown as jest.Mock;

describe( 'ProviderClient', () =>
{
    const consoleWarnSpy = jest.spyOn( console, 'warn' ).mockImplementation( () => { } );
    const consoleErrorSpy = jest.spyOn( console, 'error' ).mockImplementation( () => { } );

    beforeEach( () =>
    {
        jest.clearAllMocks();
    } );

    afterAll( () =>
    {
        consoleWarnSpy.mockRestore();
        consoleErrorSpy.mockRestore();
    } );

    it( 'sets providers and rejects unknown providers', () =>
    {
        const client = new ProviderClient();
        expect( client.getProvider() ).toBe( 'llama.cpp' );

        client.setProvider( 'claude-code' );
        expect( client.getProvider() ).toBe( 'claude-code' );

        client.setProvider( 'nope' );
        expect( client.getProvider() ).toBe( 'claude-code' );
    } );

    it( 'sends a message and returns response content', async () =>
    {
        const client = new ProviderClient();
        fetchMock.mockResolvedValueOnce( { ok: true, json: async () => ( { content: 'hi', model: 'm' } ) } as unknown );

        const res = await client.sendMessage( 'hello', [ { role: 'user', content: 'x' } ] );
        expect( res.content ).toBe( 'hi' );
        expect( res.provider ).toBe( 'llama.cpp' );
    } );

    it( 'streams tokens from SSE-style response', async () =>
    {
        const client = new ProviderClient();

        const chunks = [
            'data: {"token":"A"}\n',
            'data: {"content":"B"}\n',
            'data: [DONE]\n',
        ];

        let i = 0;
        const reader = {
            read: jest.fn( async () =>
            {
                if ( i >= chunks.length ) return { done: true, value: undefined };
                const value = Buffer.from( chunks[ i++ ]!, 'utf8' );
                return { done: false, value };
            } ),
        };

        fetchMock.mockResolvedValueOnce( {
            ok: true,
            body: { getReader: () => reader },
        } as unknown );

        const out: string[] = [];
        for await ( const token of client.streamMessage( 'hello' ) )
        {
            out.push( token );
        }

        expect( out ).toEqual( [ 'A', 'B' ] );
    } );
} );
