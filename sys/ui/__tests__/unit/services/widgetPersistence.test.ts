import { WidgetPersistenceService } from '@/services/widgetPersistence';
import type { WidgetLayout } from '@/components/widgets/WidgetGrid';

const fetchMock = global.fetch as unknown as jest.Mock;
const localStorageMock = global.localStorage as unknown as {
    getItem: jest.Mock;
    setItem: jest.Mock;
    removeItem: jest.Mock;
};

describe( 'WidgetPersistenceService', () =>
{
    const consoleWarnSpy = jest.spyOn( console, 'warn' ).mockImplementation( () => { } );
    const consoleErrorSpy = jest.spyOn( console, 'error' ).mockImplementation( () => { } );

    const layouts: WidgetLayout[] = [
        { id: 'l1', widgetId: 'w1', x: 0, y: 0, width: 1, height: 1 },
    ];

    beforeEach( () =>
    {
        jest.clearAllMocks();
        localStorageMock.getItem.mockReturnValue( null );
    } );

    afterAll( () =>
    {
        consoleWarnSpy.mockRestore();
        consoleErrorSpy.mockRestore();
    } );

    it( 'saves and loads layouts', async () =>
    {
        const svc = new WidgetPersistenceService();
        await svc.saveLayouts( layouts );
        expect( localStorageMock.setItem ).toHaveBeenCalled();

        localStorageMock.getItem.mockReturnValueOnce( null );
        await expect( svc.loadLayouts() ).resolves.toEqual( [] );

        const storedConfig = JSON.stringify( { layouts, preferences: {}, version: '0.0.0' } );
        localStorageMock.getItem.mockReturnValueOnce( storedConfig );
        const loaded = await svc.loadLayouts();
        expect( loaded ).toEqual( layouts );
        expect( consoleWarnSpy ).toHaveBeenCalled();
    } );

    it( 'returns empty layouts on parse errors', async () =>
    {
        const svc = new WidgetPersistenceService();
        localStorageMock.getItem.mockReturnValueOnce( 'not-json' );
        await expect( svc.loadLayouts() ).resolves.toEqual( [] );
    } );

    it( 'saves and loads widget preferences', async () =>
    {
        const svc = new WidgetPersistenceService();
        localStorageMock.getItem.mockReturnValueOnce( JSON.stringify( { layouts: [], preferences: {}, version: '1.0.0' } ) );

        await svc.savePreferences( 'w1', { a: 1 } );
        expect( localStorageMock.setItem ).toHaveBeenCalled();

        localStorageMock.getItem.mockReturnValueOnce( JSON.stringify( { layouts: [], preferences: { w1: { a: 1 } }, version: '1.0.0' } ) );
        const prefs = await svc.loadPreferences( 'w1' );
        expect( prefs ).toEqual( { a: 1 } );

        localStorageMock.getItem.mockReturnValueOnce( null );
        await expect( svc.loadPreferences( 'missing' ) ).resolves.toEqual( {} );
    } );

    it( 'clears local data', async () =>
    {
        const svc = new WidgetPersistenceService();
        await svc.clear();
        expect( localStorageMock.removeItem ).toHaveBeenCalled();
    } );

    it( 'syncs to and from remote', async () =>
    {
        const svc = new WidgetPersistenceService();

        fetchMock.mockResolvedValueOnce( { ok: true } as unknown );
        await svc.syncToRemote( layouts );
        expect( fetchMock ).toHaveBeenCalledWith( '/api/v1/widgets/sync', expect.objectContaining( { method: 'POST' } ) );

        fetchMock.mockResolvedValueOnce( { ok: true, json: async () => ( { layouts } ) } as unknown );
        const remote = await svc.syncFromRemote();
        expect( remote ).toEqual( layouts );

        fetchMock.mockResolvedValueOnce( { ok: false, statusText: 'nope' } as unknown );
        await expect( svc.syncFromRemote() ).resolves.toEqual( [] );
    } );
} );
