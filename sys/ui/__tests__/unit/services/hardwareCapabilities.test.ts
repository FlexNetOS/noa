import { HardwareCapabilitiesService } from '@/services/hardwareCapabilities';

describe( 'HardwareCapabilitiesService', () =>
{
    const consoleWarnSpy = jest.spyOn( console, 'warn' ).mockImplementation( () => { } );

    beforeEach( () =>
    {
        jest.clearAllMocks();
        delete ( window as unknown as Record<string, unknown> ).webkitSpeechRecognition;
        delete ( window as unknown as Record<string, unknown> ).SpeechRecognition;
        Object.defineProperty( navigator, 'mediaDevices', { value: undefined, configurable: true } );
    } );

    afterAll( () =>
    {
        consoleWarnSpy.mockRestore();
    } );

    it( 'detects webkit voice API', async () =>
    {
        Object.defineProperty( window, 'webkitSpeechRecognition', { value: function Webkit () { }, configurable: true } );
        const svc = new HardwareCapabilitiesService();

        const caps = await svc.detect();
        expect( caps.voice.available ).toBe( true );
        expect( caps.voice.api ).toBe( 'webkit' );
    } );

    it( 'detects camera and screen capture capabilities', async () =>
    {
        const enumerateDevices = jest.fn().mockResolvedValue( [ { kind: 'videoinput' } ] );
        Object.defineProperty( navigator, 'mediaDevices', {
            value: { enumerateDevices, getDisplayMedia: jest.fn() },
            configurable: true,
        } );

        const svc = new HardwareCapabilitiesService();
        const caps = await svc.detect();

        expect( caps.vision.available ).toBe( true );
        expect( caps.vision.camera ).toBe( true );
        expect( caps.vision.screenCapture ).toBe( true );

        // cached
        const caps2 = await svc.detect();
        expect( caps2 ).toBe( caps );
        expect( enumerateDevices ).toHaveBeenCalledTimes( 1 );
    } );
} );
