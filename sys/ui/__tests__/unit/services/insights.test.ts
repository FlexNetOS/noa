import { InsightsService } from '@/services/insights';
import { contextDetector } from '@/services/contextDetector';

const fetchMock = global.fetch as unknown as jest.Mock;

describe( 'InsightsService', () =>
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

    it( 'generates insights based on coding context and activity', async () =>
    {
        const svc = new InsightsService();
        contextDetector.detectContext( { type: 'file_edit', path: 'test.ts' } );

        const generated = await svc.generateInsights( { type: 'performance' } );
        expect( generated.length ).toBeGreaterThanOrEqual( 2 );

        const all = svc.getAllInsights();
        expect( all.length ).toBeGreaterThanOrEqual( 2 );
        expect( svc.getHighPriorityInsights().some( i => i.priority === 'high' ) ).toBe( true );
    } );

    it( 'supports dismissing insights', async () =>
    {
        const svc = new InsightsService();
        contextDetector.detectContext( { type: 'file_edit', path: 'test.ts' } );
        const generated = await svc.generateInsights();

        svc.dismissInsight( generated[ 0 ]!.id );
        expect( svc.getAllInsights().some( i => i.id === generated[ 0 ]!.id ) ).toBe( false );

        svc.dismissAllInsights();
        expect( svc.getAllInsights() ).toHaveLength( 0 );
    } );

    it( 'requests AI insights from backend', async () =>
    {
        const svc = new InsightsService();
        fetchMock.mockResolvedValueOnce( {
            ok: true,
            json: async () => ( { insights: [ { id: 'i1', type: 'suggestion', title: 't', message: 'm', priority: 'low', actionable: false, timestamp: 'x' } ] } ),
        } as unknown );

        const insights = await svc.requestAIInsights( 'hello' );
        expect( insights ).toHaveLength( 1 );
        expect( svc.getAllInsights().some( i => i.id === 'i1' ) ).toBe( true );
    } );
} );
