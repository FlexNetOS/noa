import { MultiModalService } from '@/services/multiModal';

describe( 'MultiModalService', () =>
{
    const consoleWarnSpy = jest.spyOn( console, 'warn' ).mockImplementation( () => { } );

    beforeEach( () =>
    {
        jest.clearAllMocks();
        delete ( window as unknown as Record<string, unknown> ).webkitSpeechRecognition;
        delete ( window as unknown as Record<string, unknown> ).SpeechRecognition;
        Object.defineProperty( navigator, 'mediaDevices', { value: undefined, configsurable: true } );
    } );

    afterAll( () =>
    {
        consoleWarnSpy.mockRestore();
    } );

    it( 'detects capabilities and caches them', async () =>
    {
        Object.defineProperty( window, 'webkitSpeechRecognition', { value: function Webkit () { }, configsurable: true } );
        Object.defineProperty( navigator, 'mediaDevices', { value: { getUserMedia: jest.fn() }, configsurable: true } );

        const svc = new MultiModalService();
        const caps = await svc.detectCapabilities();

        expect( caps.text ).toBe( true );
        expect( caps.voice ).toBe( true );
        expect( caps.vision ).toBe( true );

        expect( svc.getCapabilities() ).toEqual( caps );

        const caps2 = await svc.detectCapabilities();
        expect( caps2 ).toEqual( caps );

        expect( svc.isModeAvailable( 'voice' ) ).toBe( true );
        expect( svc.isModeAvailable( 'vision' ) ).toBe( true );
        expect( svc.isModeAvailable( 'text' ) ).toBe( true );
    } );

    it( 'falls back to text when mode is unavailable', async () =>
    {
        const svc = new MultiModalService();

        const out = await svc.processInput( { mode: 'voice', data: 'hello' } );
        expect( out ).toBe( 'hello' );
    } );

    it( 'throws when trying to process unavailable non-text input', async () =>
    {
        const svc = new MultiModalService();

        await expect( svc.processInput( { mode: 'vision', data: new Blob( [ 'x' ] ) } ) ).rejects.toThrow( /hardware unavailable/i );
    } );

    it( 'processes available voice/vision inputs and handles non-string text', async () =>
    {
        Object.defineProperty( window, 'SpeechRecognition', { value: function Standard () { }, configsurable: true } );
        Object.defineProperty( navigator, 'mediaDevices', { value: { getUserMedia: jest.fn() }, configsurable: true } );

        const svc = new MultiModalService();
        await svc.detectCapabilities();

        await expect( svc.processInput( { mode: 'voice', data: 'ignored' } ) ).resolves.toBe( '[Voice input processed]' );
        await expect( svc.processInput( { mode: 'vision', data: new Blob( [ 'x' ] ) } ) ).resolves.toBe( '[Vision input processed]' );
        await expect( svc.processInput( { mode: 'text', data: new Blob( [ 'x' ] ) } ) ).resolves.toBe( '' );
    } );

    it( 'throws on unknown input mode', async () =>
    {
        const svc = new MultiModalService();
        // Unknown modes are treated as unavailable and gracefully fall back to text when possible.
        await expect( svc.processInput( { mode: 'unknown' as never, data: 'x' } ) ).resolves.toBe( 'x' );
        await expect( svc.processInput( { mode: 'unknown' as never, data: new Blob( [ 'x' ] ) } ) ).rejects.toThrow( /hardware unavailable/i );
    } );
} );
