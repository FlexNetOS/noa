import { PresetsService } from '@/services/presets';
import type { WidgetLayout } from '@/components/widgets/WidgetGrid';

const localStorageMock = global.localStorage as unknown as {
    getItem: jest.Mock;
    setItem: jest.Mock;
};

describe( 'PresetsService', () =>
{
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
        consoleErrorSpy.mockRestore();
    } );

    it( 'creates, updates, and deletes presets', async () =>
    {
        const svc = new PresetsService();
        const preset = await svc.createPreset( 'My Preset', layouts, { theme: 'dark' } );

        expect( preset.name ).toBe( 'My Preset' );
        expect( svc.getAllPresets().length ).toBe( 1 );
        expect( localStorageMock.setItem ).toHaveBeenCalled();

        const updated = await svc.updatePreset( preset.id, { name: 'New Name' } );
        expect( updated?.name ).toBe( 'New Name' );

        const switched = await svc.switchPreset( preset.id );
        expect( switched?.id ).toBe( preset.id );
        expect( svc.getCurrentPreset()?.id ).toBe( preset.id );

        const deleted = await svc.deletePreset( preset.id );
        expect( deleted ).toBe( true );
        expect( svc.getAllPresets() ).toHaveLength( 0 );

        expect( await svc.updatePreset( 'missing', { name: 'x' } ) ).toBeNull();
        expect( await svc.deletePreset( 'missing' ) ).toBe( false );
        expect( svc.getCurrentPreset() ).toBeNull();
        expect( svc.exportPreset( 'missing' ) ).toBeNull();
    } );

    it( 'exports and imports presets', async () =>
    {
        const svc = new PresetsService();
        const preset = await svc.createPreset( 'Export Me', layouts );
        const json = svc.exportPreset( preset.id );
        expect( json ).toContain( 'Export Me' );

        const imported = await svc.importPreset( json! );
        expect( imported?.id ).toBe( preset.id );
    } );

    it( 'rejects invalid preset json', async () =>
    {
        const svc = new PresetsService();
        const imported = await svc.importPreset( '{"bad":true}' );
        expect( imported ).toBeNull();
    } );

    it( 'loads presets from storage and rejects structurally invalid presets', async () =>
    {
        const svc = new PresetsService();
        const stored = {
            presets: {
                p1: {
                    id: 'p1',
                    name: 'Stored',
                    widgetLayouts: layouts,
                    settings: {},
                    createdAt: '2020-01-01T00:00:00.000Z',
                    updatedAt: '2020-01-01T00:00:00.000Z',
                },
            },
            currentPresetId: 'p1',
        };
        localStorageMock.getItem.mockReturnValueOnce( JSON.stringify( stored ) );
        await svc.loadPresets();
        expect( svc.getCurrentPreset()?.id ).toBe( 'p1' );

        // Missing required fields should be rejected
        await expect( svc.importPreset( '{"id":"x","name":"n"}' ) ).resolves.toBeNull();
    } );
} );
