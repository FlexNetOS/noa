'use client';

import { useEffect, useState } from 'react';
import { apiClient } from '@/lib/api';
import { FileText, Download, Eye } from 'lucide-react';
import MainLayout from '@/components/layout/MainLayout';

interface Artifact
{
  id: string;
  type: string;
  name: string;
  created_at: string;
}

export default function ArtifactsPage ()
{
  const [ artifacts, setArtifacts ] = useState<Artifact[]>( [] );
  const [ loading, setLoading ] = useState( true );

  useEffect( () =>
  {
    const loadArtifacts = async () =>
    {
      try
      {
        const response = await apiClient.getArtifacts<Artifact>();
        setArtifacts( response.artifacts );
      } catch ( error )
      {
        console.error( 'Failed to load artifacts:', error );
      } finally
      {
        setLoading( false );
      }
    };

    loadArtifacts();
  }, [] );

  return (
    <MainLayout>
      <div className="space-y-6">
        <div className="flex items-center gap-3">
          <FileText className="w-8 h-8 text-blue-400" />
          <h1 className="text-3xl font-bold text-slate-100">Artifacts Explorer</h1>
        </div>

        { loading ? (
          <div className="text-center py-12 text-slate-400">Loading artifacts...</div>
        ) : (
          <div className="bg-slate-800/50 backdrop-blur-sm rounded-2xl border border-slate-700 overflow-hidden">
            <div className="overflow-x-auto">
              <table className="w-full">
                <thead className="bg-slate-900/50">
                  <tr>
                    <th className="px-6 py-4 text-left text-sm font-semibold text-slate-300">Name</th>
                    <th className="px-6 py-4 text-left text-sm font-semibold text-slate-300">Type</th>
                    <th className="px-6 py-4 text-left text-sm font-semibold text-slate-300">Created</th>
                    <th className="px-6 py-4 text-left text-sm font-semibold text-slate-300">Actions</th>
                  </tr>
                </thead>
                <tbody className="divide-y divide-slate-700">
                  { artifacts.length === 0 ? (
                    <tr>
                      <td colSpan={ 4 } className="px-6 py-12 text-center text-slate-400">
                        No artifacts found
                      </td>
                    </tr>
                  ) : (
                    artifacts.map( ( artifact ) => (
                      <tr key={ artifact.id } className="hover:bg-slate-800/50 transition-colors">
                        <td className="px-6 py-4 text-sm text-slate-300 font-medium">{ artifact.name }</td>
                        <td className="px-6 py-4 text-sm text-slate-400 capitalize">{ artifact.type }</td>
                        <td className="px-6 py-4 text-sm text-slate-400">
                          { new Date( artifact.created_at ).toLocaleString() }
                        </td>
                        <td className="px-6 py-4">
                          <div className="flex items-center gap-2">
                            <button className="p-2 hover:bg-slate-700 rounded-lg transition-colors">
                              <Eye className="w-4 h-4 text-slate-400" />
                            </button>
                            <button className="p-2 hover:bg-slate-700 rounded-lg transition-colors">
                              <Download className="w-4 h-4 text-slate-400" />
                            </button>
                          </div>
                        </td>
                      </tr>
                    ) )
                  ) }
                </tbody>
              </table>
            </div>
          </div>
        ) }
      </div>
    </MainLayout>
  );
}

