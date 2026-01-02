/**
 * DataTable Widget
 * 
 * Sortable, filterable table with pagination.
 * Designed for displaying tabular data from ML experiments,
 * model metrics, or any structured data.
 * 
 * Rust Translation (Dioxus):
 * Use egui_extras::Table or custom Dioxus table component
 */

'use client';

import React, { useState, useMemo } from 'react';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { ArrowUp, ArrowDown, Search, ChevronLeft, ChevronRight } from 'lucide-react';
import { motion, AnimatePresence } from 'framer-motion';

export interface DataTableProps {
  title?: string;
  description?: string;
  columns: Array<{
    key: string;
    label: string;
    sortable?: boolean;
  }>;
  data: Array<Record<string, any>>;
  pageSize?: number;
  searchable?: boolean;
  className?: string;
}

export function DataTable({
  title = 'Data Table',
  description,
  columns,
  data,
  pageSize = 10,
  searchable = true,
  className = '',
}: DataTableProps) {
  const [sortColumn, setSortColumn] = useState<string | null>(null);
  const [sortDirection, setSortDirection] = useState<'asc' | 'desc'>('asc');
  const [searchQuery, setSearchQuery] = useState('');
  const [currentPage, setCurrentPage] = useState(0);

  // Filter and sort data
  const processedData = useMemo(() => {
    let filtered = data;

    // Apply search filter
    if (searchQuery) {
      filtered = data.filter(row =>
        Object.values(row).some(value =>
          String(value).toLowerCase().includes(searchQuery.toLowerCase())
        )
      );
    }

    // Apply sorting
    if (sortColumn) {
      filtered = [...filtered].sort((a, b) => {
        const aVal = a[sortColumn];
        const bVal = b[sortColumn];
        
        if (aVal === bVal) return 0;
        
        const comparison = aVal > bVal ? 1 : -1;
        return sortDirection === 'asc' ? comparison : -comparison;
      });
    }

    return filtered;
  }, [data, searchQuery, sortColumn, sortDirection]);

  // Pagination
  const totalPages = Math.ceil(processedData.length / pageSize);
  const paginatedData = processedData.slice(
    currentPage * pageSize,
    (currentPage + 1) * pageSize
  );

  const handleSort = (columnKey: string) => {
    if (sortColumn === columnKey) {
      setSortDirection(sortDirection === 'asc' ? 'desc' : 'asc');
    } else {
      setSortColumn(columnKey);
      setSortDirection('asc');
    }
  };

  return (
    <Card className={className}>
      <CardHeader>
        <CardTitle>{title}</CardTitle>
        {description && <CardDescription>{description}</CardDescription>}
        
        {searchable && (
          <div className="flex items-center gap-2 pt-2">
            <Search className="w-4 h-4 text-muted-foreground" />
            <Input
              placeholder="Search..."
              value={searchQuery}
              onChange={(e) => {
                setSearchQuery(e.target.value);
                setCurrentPage(0); // Reset to first page on search
              }}
              className="max-w-sm"
            />
          </div>
        )}
      </CardHeader>
      
      <CardContent>
        <div className="overflow-x-auto">
          <table className="w-full border-collapse">
            <thead>
              <tr className="border-b">
                {columns.map((column) => (
                  <th
                    key={column.key}
                    className={`px-4 py-2 text-left font-medium text-sm ${
                      column.sortable !== false ? 'cursor-pointer hover:bg-muted' : ''
                    }`}
                    onClick={() => column.sortable !== false && handleSort(column.key)}
                  >
                    <div className="flex items-center gap-2">
                      {column.label}
                      {column.sortable !== false && sortColumn === column.key && (
                        <motion.div
                          initial={{ opacity: 0, scale: 0.5 }}
                          animate={{ opacity: 1, scale: 1 }}
                          transition={{ duration: 0.2 }}
                        >
                          {sortDirection === 'asc' ? (
                            <ArrowUp className="w-4 h-4" />
                          ) : (
                            <ArrowDown className="w-4 h-4" />
                          )}
                        </motion.div>
                      )}
                    </div>
                  </th>
                ))}
              </tr>
            </thead>
            
            <tbody>
              <AnimatePresence mode="wait">
                {paginatedData.map((row, rowIndex) => (
                  <motion.tr
                    key={rowIndex}
                    className="border-b hover:bg-muted/50 transition-colors"
                    initial={{ opacity: 0, y: -10 }}
                    animate={{ opacity: 1, y: 0 }}
                    exit={{ opacity: 0, y: 10 }}
                    transition={{ duration: 0.2, delay: rowIndex * 0.03 }}
                  >
                    {columns.map((column) => (
                      <td key={column.key} className="px-4 py-2 text-sm">
                        {String(row[column.key] ?? '-')}
                      </td>
                    ))}
                  </motion.tr>
                ))}
              </AnimatePresence>
            </tbody>
          </table>
        </div>

        {/* Pagination */}
        {totalPages > 1 && (
          <div className="flex items-center justify-between mt-4">
            <div className="text-sm text-muted-foreground">
              Showing {currentPage * pageSize + 1}-
              {Math.min((currentPage + 1) * pageSize, processedData.length)} of{' '}
              {processedData.length} rows
            </div>
            
            <div className="flex items-center gap-2">
              <Button
                variant="outline"
                size="sm"
                onClick={() => setCurrentPage(Math.max(0, currentPage - 1))}
                disabled={currentPage === 0}
              >
                <ChevronLeft className="w-4 h-4" />
                Previous
              </Button>
              
              <div className="text-sm">
                Page {currentPage + 1} of {totalPages}
              </div>
              
              <Button
                variant="outline"
                size="sm"
                onClick={() => setCurrentPage(Math.min(totalPages - 1, currentPage + 1))}
                disabled={currentPage >= totalPages - 1}
              >
                Next
                <ChevronRight className="w-4 h-4" />
              </Button>
            </div>
          </div>
        )}

        {paginatedData.length === 0 && (
          <div className="text-center py-8 text-muted-foreground">
            No data to display
          </div>
        )}
      </CardContent>
    </Card>
  );
}

/**
 * Rust/Dioxus Translation:
 * 
 * ```rust
 * #[component]
 * pub fn DataTable<G: Clone + 'static>(
 *     cx: Scope,
 *     title: String,
 *     columns: Vec<Column>,
 *     data: Vec<HashMap<String, String>>,
 * ) -> Element {
 *     let sort_column = use_state(cx, || None);
 *     let sort_direction = use_state(cx, || SortDirection::Asc);
 *     let search_query = use_state(cx, || String::new());
 *     
 *     // Filter and sort logic
 *     let processed_data = use_memo(cx, (&search_query, &sort_column), |_| {
 *         filter_and_sort(&data, &search_query, &sort_column, &sort_direction)
 *     });
 *     
 *     cx.render(rsx! {
 *         div { class: "data-table",
 *             table {
 *                 thead {
 *                     tr {
 *                         for column in &columns {
 *                             th { onclick: move |_| handle_sort(column.key),
 *                                 "{column.label}"
 *                             }
 *                         }
 *                     }
 *                 }
 *                 tbody {
 *                     for row in processed_data.iter() {
 *                         tr {
 *                             for column in &columns {
 *                                 td { "{row[&column.key]}" }
 *                             }
 *                         }
 *                     }
 *                 }
 *             }
 *         }
 *     })
 * }
 * ```
 */