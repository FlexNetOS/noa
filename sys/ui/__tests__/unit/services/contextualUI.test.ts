import { ContextualUIService } from '@/services/contextualUI';
import { contextDetector } from '@/services/contextDetector';

describe( 'ContextualUIService', () =>
{
    const consoleErrorSpy = jest.spyOn( console, 'error' ).mockImplementation( () => { } );

    beforeEach( () =>
    {
        jest.clearAllMocks();
        contextDetector.reset();
    } );

    afterAll( () =>
    {
        consoleErrorSpy.mockRestore();
    } );

    it( 'adapts state based on detected context', () =>
    {
        const svc = new ContextualUIService();

        const state = svc.adaptToActivity( { type: 'file_edit', path: 'test.ts' } );

        expect( state.context?.type ).toBe( 'coding' );
        expect( state.suggestedTools ).toContain( 'Code Editor' );
        expect( state.visibleComponents ).toEqual( [ 'CodeEditor', 'Terminal', 'FileExplorer' ] );
    } );

    it( 'captures highlight adaptations and supports reset', () =>
    {
        const svc = new ContextualUIService();

        const state = svc.adaptToActivity( { type: 'search', keywords: [ 'research' ] } );
        expect( state.context?.type ).toBe( 'research' );
        expect( state.highlightedComponents ).toContain( 'Search' );

        svc.reset();
        expect( svc.getState().context ).toBeNull();
        expect( svc.getState().visibleComponents ).toEqual( [] );
    } );

    it( 'notifies subscribers and supports unsubscribe', () =>
    {
        const svc = new ContextualUIService();
        const listener = jest.fn();

        const unsubscribe = svc.subscribe( listener );
        svc.adaptToActivity( { type: 'task_created', keywords: [ 'task' ] } );
        expect( listener ).toHaveBeenCalled();

        listener.mockClear();
        unsubscribe();
        svc.setVisibleComponents( [ 'X' ] );
        expect( listener ).not.toHaveBeenCalled();
    } );

    it( 'does not crash if a subscriber throws', () =>
    {
        const svc = new ContextualUIService();
        const bad = jest.fn( () =>
        {
            throw new Error( 'boom' );
        } );
        const good = jest.fn();

        svc.subscribe( bad );
        svc.subscribe( good );

        expect( () => svc.setHighlightedComponents( [ 'A' ] ) ).not.toThrow();
        expect( good ).toHaveBeenCalled();
    } );
} );
