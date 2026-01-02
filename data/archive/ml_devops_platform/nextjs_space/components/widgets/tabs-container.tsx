'use client';

import React, { useState } from 'react';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs';
import { Layers } from 'lucide-react';

/**
 * TabsContainer Widget - Tabbed interface for nested widgets
 * 
 * Features:
 * - Multiple tabs with nested content
 * - Tab activation state
 * - Lazy loading of tab content
 * - Support for nested widgets in each tab
 * 
 * Rust/Dioxus Translation:
 * - Use Dioxus Router or custom tab component
 * - Implement with match statements for active tab
 * - Map to Dioxus hooks for state management
 */

interface TabConfig {
  id: string;
  label: string;
  widgetId: string;
  icon?: React.ReactNode;
}

interface TabsContainerConfig {
  title?: string;
  tabs: TabConfig[];
  defaultTab?: string;
}

interface TabsContainerProps {
  config: TabsContainerConfig;
  className?: string;
  children?: React.ReactNode;
}

export function TabsContainer({ config, className = '', children }: TabsContainerProps) {
  const { title = 'Tabs', tabs, defaultTab } = config;
  const [activeTab, setActiveTab] = useState(defaultTab || tabs[0]?.id || '');

  // Convert children to array
  const childArray = React.Children.toArray(children);

  return (
    <Card className={className}>
      {title && (
        <CardHeader>
          <CardTitle className="flex items-center gap-2">
            <Layers className="h-5 w-5" />
            {title}
          </CardTitle>
        </CardHeader>
      )}
      <CardContent>
        <Tabs value={activeTab} onValueChange={setActiveTab}>
          <TabsList className="grid w-full" style={{ gridTemplateColumns: `repeat(${tabs.length}, 1fr)` }}>
            {tabs.map((tab) => (
              <TabsTrigger key={tab.id} value={tab.id} className="flex items-center gap-2">
                {tab.icon}
                {tab.label}
              </TabsTrigger>
            ))}
          </TabsList>

          {tabs.map((tab, index) => (
            <TabsContent key={tab.id} value={tab.id} className="mt-4">
              {childArray[index] || (
                <div className="text-center text-muted-foreground py-8">
                  No content for this tab
                </div>
              )}
            </TabsContent>
          ))}
        </Tabs>
      </CardContent>
    </Card>
  );
}
