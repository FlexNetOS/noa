'use client';

import { useEffect, useState } from 'react';
import { apiClient } from '@/lib/api';
import { Briefcase, Clock, CheckCircle, XCircle, Loader } from 'lucide-react';
import MainLayout from '@/components/layout/MainLayout';

interface Job
{
  id: string;
  type: string;
  status: string;
  created_at: string;
}

export default function JobsPage ()
{
  const [ jobs, setJobs ] = useState<Job[]>( [] );
  const [ loading, setLoading ] = useState( true );

  useEffect( () =>
  {
    const loadJobs = async () =>
    {
      try
      {
        const response = await apiClient.getJobs<Job>();
        setJobs( response.jobs );
      } catch ( error )
      {
        console.error( 'Failed to load jobs:', error );
      } finally
      {
        setLoading( false );
      }
    };

    loadJobs();
    const interval = setInterval( loadJobs, 5000 );
    return () => clearInterval( interval );
  }, [] );

  const getStatusIcon = ( status: string ) =>
  {
    switch ( status.toLowerCase() )
    {
      case 'completed':
        return <CheckCircle className="w-5 h-5 text-emerald-400" />;
      case 'failed':
        return <XCircle className="w-5 h-5 text-red-400" />;
      case 'running':
        return <Loader className="w-5 h-5 text-blue-400 animate-spin" />;
      default:
        return <Clock className="w-5 h-5 text-slate-400" />;
    }
  };

  return (
    <MainLayout>
      <div className="space-y-6">
        <div className="flex items-center gap-3">
          <Briefcase className="w-8 h-8 text-blue-400" />
          <h1 className="text-3xl font-bold text-slate-100">Jobs Dashboard</h1>
        </div>

        { loading ? (
          <div className="text-center py-12 text-slate-400">Loading jobs...</div>
        ) : (
          <div className="bg-slate-800/50 backdrop-blur-sm rounded-2xl border border-slate-700 overflow-hidden">
            <div className="overflow-x-auto">
              <table className="w-full">
                <thead className="bg-slate-900/50">
                  <tr>
                    <th className="px-6 py-4 text-left text-sm font-semibold text-slate-300">ID</th>
                    <th className="px-6 py-4 text-left text-sm font-semibold text-slate-300">Type</th>
                    <th className="px-6 py-4 text-left text-sm font-semibold text-slate-300">Status</th>
                    <th className="px-6 py-4 text-left text-sm font-semibold text-slate-300">Created</th>
                  </tr>
                </thead>
                <tbody className="divide-y divide-slate-700">
                  { jobs.length === 0 ? (
                    <tr>
                      <td colSpan={ 4 } className="px-6 py-12 text-center text-slate-400">
                        No jobs found
                      </td>
                    </tr>
                  ) : (
                    jobs.map( ( job ) => (
                      <tr key={ job.id } className="hover:bg-slate-800/50 transition-colors">
                        <td className="px-6 py-4 text-sm text-slate-300 font-mono">
                          { job.id.slice( 0, 8 ) }...
                        </td>
                        <td className="px-6 py-4 text-sm text-slate-300">{ job.type }</td>
                        <td className="px-6 py-4">
                          <div className="flex items-center gap-2">
                            { getStatusIcon( job.status ) }
                            <span className="text-sm text-slate-300 capitalize">{ job.status }</span>
                          </div>
                        </td>
                        <td className="px-6 py-4 text-sm text-slate-400">
                          { new Date( job.created_at ).toLocaleString() }
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

