import { SettingsSyncService, type SyncConflict } from '@/services/settingsSync';

const fetchMock = global.fetch as unknown as jest.Mock;
const localStorageMock = global.localStorage as unknown as {
    getItem: jest.Mock;
};

describe( 'SettingsSyncService', () =>
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

    it( 'returns an error when sync is disabled', async () =>
    {
        const svc = new SettingsSyncService();
        svc.setEnabled( false );

        const res = await svc.syncToRemote( { a: 1 } );
        expect( res.success ).toBe( false );
        expect( res.error ).toMatch( /disabled/i );
    } );

    it( 'syncs to remote and returns synced keys', async () =>
    {
        const svc = new SettingsSyncService();
        fetchMock.mockResolvedValueOnce( { ok: true, json: async () => ( { conflicts: [] } ) } as unknown );

        const res = await svc.syncToRemote( { a: 1, b: 2 } );
        expect( res.success ).toBe( true );
        expect( res.syncedKeys ).toEqual( [ 'a', 'b' ] );
    } );

    it( 'returns an error when syncFromRemote is disabled', async () =>
    {
        const svc = new SettingsSyncService();
        svc.setEnabled( false );
        const res = await svc.syncFromRemote();
        expect( res.success ).toBe( false );
        expect( res.error ).toMatch( /disabled/i );
    } );

    it( 'detects conflicts when syncing from remote', async () =>
    {
        const svc = new SettingsSyncService();
        localStorageMock.getItem.mockReturnValueOnce( JSON.stringify( { theme: 'dark' } ) );
        fetchMock.mockResolvedValueOnce( { ok: true, json: async () => ( { settings: { theme: 'light' } } ) } as unknown );

        const res = await svc.syncFromRemote();
        expect( res.success ).toBe( true );
        expect( res.conflicts ).toHaveLength( 1 );
        expect( res.conflicts?.[ 0 ]?.key ).toBe( 'theme' );
    } );

    it( 'handles syncFromRemote without conflicts', async () =>
    {
        const svc = new SettingsSyncService();
        localStorageMock.getItem.mockReturnValueOnce( JSON.stringify( {} ) );
        fetchMock.mockResolvedValueOnce( { ok: true, json: async () => ( { settings: { theme: 'light' } } ) } as unknown );

        const res = await svc.syncFromRemote();
        expect( res.success ).toBe( true );
        expect( res.conflicts ).toBeUndefined();
    } );

    it( 'resolves conflicts via backend', async () =>
    {
        const svc = new SettingsSyncService();
        fetchMock.mockResolvedValueOnce( { ok: true } as unknown );

        const res = await svc.resolveConflicts(
            [ { key: 'k', localValue: 1, remoteValue: 2, timestamp: 't' } ],
            { k: 2 }
        );
        expect( res.success ).toBe( true );
        expect( res.syncedKeys ).toEqual( [ 'k' ] );
    } );

    it( 'auto-resolves conflicts based on strategy', () =>
    {
        const svc = new SettingsSyncService();
        const conflicts: SyncConflict[] = [ { key: 'k', localValue: { a: 1 }, remoteValue: { b: 2 }, timestamp: 't' } ];

        svc.setConflictResolution( 'merge' );
        const merged = svc.autoResolveConflicts( conflicts );
        expect( merged.k ).toEqual( { a: 1, b: 2 } );

        svc.setConflictResolution( 'manual' );
        const manual = svc.autoResolveConflicts( conflicts );
        expect( manual.k ).toEqual( { a: 1 } );

        svc.setConflictResolution( 'last_write_wins' );
        const lww = svc.autoResolveConflicts( conflicts );
        expect( lww.k ).toEqual( { b: 2 } );
    } );
} );
