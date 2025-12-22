'use client';

import { useEffect, useState } from 'react';
import { apiClient } from '@/lib/api';
import { Package, CheckCircle, XCircle, Clock } from 'lucide-react';
import MainLayout from '@/components/layout/MainLayout';

interface Capsule
{
  id: string;
  name: string;
  status: string;
  created_at: string;
}

export default function CapsulesPage ()
{
  const [ capsules, setCapsules ] = useState<Capsule[]>( [] );
  const [ loading, setLoading ] = useState( true );

  useEffect( () =>
  {
    const loadCapsules = async () =>
    {
      try
      {
        const response = await apiClient.getCapsules<Capsule>();
        setCapsules( response.capsules );
      } catch ( error )
      {
        console.error( 'Failed to load capsules:', error );
      } finally
      {
        setLoading( false );
      }
    };

    loadCapsules();
  }, [] );

  const getStatusIcon = ( status: string ) =>
  {
    switch ( status.toLowerCase() )
    {
      case 'active':
        return <CheckCircle className="w-5 h-5 text-emerald-400" />;
      case 'inactive':
        return <XCircle className="w-5 h-5 text-red-400" />;
      default:
        return <Clock className="w-5 h-5 text-slate-400" />;
    }
  };

  return (
    <MainLayout>
      <div className="space-y-6">
        <div className="flex items-center gap-3">
          <Package className="w-8 h-8 text-blue-400" />
          <h1 className="text-3xl font-bold text-slate-100">Capsules</h1>
        </div>

        { loading ? (
          <div className="text-center py-12 text-slate-400">Loading capsules...</div>
        ) : (
          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
            { capsules.length === 0 ? (
              <div className="col-span-full text-center py-12 text-slate-400">
                No capsules found
              </div>
            ) : (
              capsules.map( ( capsule ) => (
                <div
                  key={ capsule.id }
                  className="bg-slate-800/50 backdrop-blur-sm rounded-xl border border-slate-700 p-6 hover:border-slate-600 transition-colors"
                >
                  <div className="flex items-center justify-between mb-4">
                    <h3 className="text-lg font-semibold text-slate-100">{ capsule.name }</h3>
                    { getStatusIcon( capsule.status ) }
                  </div>
                  <div className="text-sm text-slate-400">
                    Created: { new Date( capsule.created_at ).toLocaleDateString() }
                  </div>
                </div>
              ) )
            ) }
          </div>
        ) }
      </div>
    </MainLayout>
  );
}

