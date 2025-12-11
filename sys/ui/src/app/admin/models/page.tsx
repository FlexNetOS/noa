'use client';

import { useEffect, useState } from 'react';
import { apiClient } from '@/lib/api';
import { Cpu, CheckCircle, XCircle, Loader } from 'lucide-react';
import MainLayout from '@/components/layout/MainLayout';

interface Model {
  id: string;
  name: string;
  provider: string;
  status: string;
}

export default function ModelsPage() {
  const [models, setModels] = useState<Model[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    const loadModels = async () => {
      try {
        const response = await apiClient.getModels();
        setModels(response.models);
      } catch (error) {
        console.error('Failed to load models:', error);
      } finally {
        setLoading(false);
      }
    };

    loadModels();
  }, []);

  const getStatusIcon = (status: string) => {
    switch (status.toLowerCase()) {
      case 'loaded':
        return <CheckCircle className="w-5 h-5 text-emerald-400" />;
      case 'loading':
        return <Loader className="w-5 h-5 text-blue-400 animate-spin" />;
      case 'error':
        return <XCircle className="w-5 h-5 text-red-400" />;
      default:
        return <Cpu className="w-5 h-5 text-slate-400" />;
    }
  };

  return (
    <MainLayout>
      <div className="space-y-6">
        <div className="flex items-center gap-3">
          <Cpu className="w-8 h-8 text-blue-400" />
          <h1 className="text-3xl font-bold text-slate-100">Model Registry</h1>
        </div>

        {loading ? (
          <div className="text-center py-12 text-slate-400">Loading models...</div>
        ) : (
          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
            {models.length === 0 ? (
              <div className="col-span-full text-center py-12 text-slate-400">
                No models found
              </div>
            ) : (
              models.map((model) => (
                <div
                  key={model.id}
                  className="bg-slate-800/50 backdrop-blur-sm rounded-xl border border-slate-700 p-6 hover:border-slate-600 transition-colors"
                >
                  <div className="flex items-center justify-between mb-4">
                    <h3 className="text-lg font-semibold text-slate-100">{model.name}</h3>
                    {getStatusIcon(model.status)}
                  </div>
                  <div className="text-sm text-slate-400">
                    Provider: {model.provider}
                  </div>
                  <div className="text-sm text-slate-400 mt-2">
                    Status: <span className="capitalize">{model.status}</span>
                  </div>
                </div>
              ))
            )}
          </div>
        )}
      </div>
    </MainLayout>
  );
}

