import { ContextPersistenceService, type ConversationContext } from '@/services/contextPersistence';

const fetchMock = global.fetch as unknown as jest.Mock;
const localStorageMock = global.localStorage as unknown as {
    getItem: jest.Mock;
    setItem: jest.Mock;
    removeItem: jest.Mock;
};

describe( 'ContextPersistenceService', () =>
{
    const consoleErrorSpy = jest.spyOn( console, 'error' ).mockImplementation( () => { } );

    beforeEach( () =>
    {
        jest.clearAllMocks();
        localStorageMock.getItem.mockReturnValue( null );
    } );

    afterAll( () =>
    {
        consoleErrorSpy.mockRestore();
    } );

    it( 'saves a new context to localStorage', async () =>
    {
        const svc = new ContextPersistenceService();
        const context: ConversationContext = {
            id: 'c1',
            messages: [ { role: 'user', content: 'hi', timestamp: '2020-01-01T00:00:00.000Z' } ],
            createdAt: '2020-01-01T00:00:00.000Z',
            updatedAt: '2020-01-01T00:00:00.000Z',
        };

        await svc.saveContext( context );

        expect( localStorageMock.setItem ).toHaveBeenCalledTimes( 1 );
        const [ , stored ] = localStorageMock.setItem.mock.calls[ 0 ] as [ string, string ];
        const parsed = JSON.parse( stored ) as ConversationContext[];
        expect( parsed ).toHaveLength( 1 );
        expect( parsed[ 0 ]?.id ).toBe( 'c1' );
    } );

    it( 'updates an existing context and refreshes updatedAt', async () =>
    {
        const svc = new ContextPersistenceService();

        const existing: ConversationContext = {
            id: 'c1',
            messages: [],
            createdAt: '2020-01-01T00:00:00.000Z',
            updatedAt: '2020-01-01T00:00:00.000Z',
        };
        localStorageMock.getItem.mockReturnValueOnce( JSON.stringify( [ existing ] ) );

        const updated: ConversationContext = { ...existing, messages: [ { role: 'user', content: 'x', timestamp: '2020-01-01T00:00:00.000Z' } ] };
        await svc.saveContext( updated );

        const [ , stored ] = localStorageMock.setItem.mock.calls[ 0 ] as [ string, string ];
        const parsed = JSON.parse( stored ) as ConversationContext[];
        expect( parsed[ 0 ]?.messages ).toHaveLength( 1 );
        expect( parsed[ 0 ]?.updatedAt ).not.toBe( existing.updatedAt );
    } );

    it( 'loads a context by id', async () =>
    {
        const svc = new ContextPersistenceService();
        const existing: ConversationContext = {
            id: 'c1',
            messages: [],
            createdAt: '2020-01-01T00:00:00.000Z',
            updatedAt: '2020-01-01T00:00:00.000Z',
        };
        localStorageMock.getItem.mockReturnValueOnce( JSON.stringify( [ existing ] ) );

        const loaded = await svc.loadContext( 'c1' );
        expect( loaded?.id ).toBe( 'c1' );
    } );

    it( 'deletes a context by id', async () =>
    {
        const svc = new ContextPersistenceService();
        const existing: ConversationContext[] = [
            {
                id: 'c1',
                messages: [],
                createdAt: '2020-01-01T00:00:00.000Z',
                updatedAt: '2020-01-01T00:00:00.000Z',
            },
            {
                id: 'c2',
                messages: [],
                createdAt: '2020-01-01T00:00:00.000Z',
                updatedAt: '2020-01-01T00:00:00.000Z',
            },
        ];
        localStorageMock.getItem.mockReturnValueOnce( JSON.stringify( existing ) );

        await svc.deleteContext( 'c1' );

        const [ , stored ] = localStorageMock.setItem.mock.calls[ 0 ] as [ string, string ];
        const parsed = JSON.parse( stored ) as ConversationContext[];
        expect( parsed.map( c => c.id ) ).toEqual( [ 'c2' ] );
    } );

    it( 'syncs context to remote', async () =>
    {
        const svc = new ContextPersistenceService();
        fetchMock.mockResolvedValueOnce( { ok: true } as unknown );

        await svc.syncToRemote( {
            id: 'c1',
            messages: [],
            createdAt: '2020-01-01T00:00:00.000Z',
            updatedAt: '2020-01-01T00:00:00.000Z',
        } );

        expect( fetchMock ).toHaveBeenCalledWith( '/api/v1/context/sync', expect.objectContaining( { method: 'POST' } ) );
    } );

    it( 'syncs contexts from remote', async () =>
    {
        const svc = new ContextPersistenceService();
        fetchMock.mockResolvedValueOnce( { ok: true, json: async () => ( { contexts: [ { id: 'c1' } ] } ) } as unknown );

        const contexts = await svc.syncFromRemote();
        expect( contexts ).toHaveLength( 1 );
    } );
} );
