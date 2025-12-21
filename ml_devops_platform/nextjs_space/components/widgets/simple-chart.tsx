'use client';

/**
 * SimpleChart Widget - Basic data visualization
 * 
 * Rust/Dioxus equivalent:
 * - Use plotters or egui for chart rendering
 * - Can also use charming (Rust wrapper for ECharts)
 */

import { Card } from '@/components/ui/card';
import { BarChart3 } from 'lucide-react';
import {
  BarChart,
  Bar,
  XAxis,
  YAxis,
  CartesianGrid,
  Tooltip,
  ResponsiveContainer,
  LineChart,
  Line,
} from 'recharts';

type ChartType = 'bar' | 'line';

interface ChartData {
  name: string;
  value: number;
  [key: string]: any;
}

interface SimpleChartProps {
  title?: string;
  data: ChartData[];
  type?: ChartType;
  className?: string;
}

export function SimpleChart({
  title,
  data,
  type = 'bar',
  className = '',
}: SimpleChartProps) {
  const safeData = data ?? [];

  return (
    <Card className={`p-4 ${className}`}>
      <div className="flex items-center gap-2 mb-4">
        <BarChart3 className="w-5 h-5 text-purple-500" />
        {title && (
          <h3 className="text-lg font-semibold text-gray-800 dark:text-gray-200">
            {title}
          </h3>
        )}
      </div>
      <ResponsiveContainer width="100%" height={250}>
        {type === 'bar' ? (
          <BarChart data={safeData}>
            <XAxis
              dataKey="name"
              tickLine={false}
              tick={{ fontSize: 10 }}
              stroke="#888"
            />
            <YAxis
              tickLine={false}
              tick={{ fontSize: 10 }}
              stroke="#888"
            />
            <Tooltip
              contentStyle={{
                backgroundColor: 'rgba(0, 0, 0, 0.8)',
                border: 'none',
                borderRadius: '8px',
                fontSize: '11px',
              }}
            />
            <Bar dataKey="value" fill="#60B5FF" radius={[4, 4, 0, 0]} />
          </BarChart>
        ) : (
          <LineChart data={safeData}>
            <XAxis
              dataKey="name"
              tickLine={false}
              tick={{ fontSize: 10 }}
              stroke="#888"
            />
            <YAxis
              tickLine={false}
              tick={{ fontSize: 10 }}
              stroke="#888"
            />
            <Tooltip
              contentStyle={{
                backgroundColor: 'rgba(0, 0, 0, 0.8)',
                border: 'none',
                borderRadius: '8px',
                fontSize: '11px',
              }}
            />
            <Line
              type="monotone"
              dataKey="value"
              stroke="#A19AD3"
              strokeWidth={2}
              dot={{ fill: '#A19AD3', r: 4 }}
            />
          </LineChart>
        )}
      </ResponsiveContainer>
    </Card>
  );
}
