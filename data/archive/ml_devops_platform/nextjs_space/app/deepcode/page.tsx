/**
 * DeepCode: Agentic Coding Interface
 * Paper2Code, Text2Web, Text2Backend workflows
 */

'use client';

import { useState } from 'react';
import { motion } from 'framer-motion';
import { 
  FileText, 
  Globe, 
  Server, 
  Sparkles, 
  Code2, 
  Play,
  Download,
  Copy,
  CheckCircle2,
  Loader2,
  AlertCircle,
} from 'lucide-react';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Textarea } from '@/components/ui/textarea';
import { Input } from '@/components/ui/input';
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs';
import { Label } from '@/components/ui/label';
import { useSonaStream } from '@/lib/hooks/use-sona';
import { createPaper2CodeWorkflow, createText2WebWorkflow, createText2BackendWorkflow } from '@/lib/sona/workflows';
import { WorkflowDefinition } from '@/lib/sona/types';
import { WorkflowMonitor } from '@/components/sona/workflow-monitor';

export default function DeepCodePage() {
  const [activeTab, setActiveTab] = useState<'paper2code' | 'text2web' | 'text2backend'>('paper2code');
  
  // Paper2Code state
  const [paperTitle, setPaperTitle] = useState('');
  const [paperContent, setPaperContent] = useState('');
  
  // Text2Web state
  const [webProjectName, setWebProjectName] = useState('');
  const [webRequirements, setWebRequirements] = useState('');
  
  // Text2Backend state
  const [backendProjectName, setBackendProjectName] = useState('');
  const [backendRequirements, setBackendRequirements] = useState('');
  
  // Workflow execution
  const { executeWithStream, isStreaming, events, result, error } = useSonaStream();
  
  const [currentWorkflow, setCurrentWorkflow] = useState<WorkflowDefinition | null>(null);
  
  const handlePaper2Code = async () => {
    if (!paperTitle || !paperContent) {
      alert('Please provide both paper title and content');
      return;
    }
    
    const workflow = createPaper2CodeWorkflow(
      Date.now().toString(),
      paperTitle,
      paperContent
    );
    
    setCurrentWorkflow(workflow);
    await executeWithStream(workflow);
  };
  
  const handleText2Web = async () => {
    if (!webProjectName || !webRequirements) {
      alert('Please provide both project name and requirements');
      return;
    }
    
    const workflow = createText2WebWorkflow(
      Date.now().toString(),
      webProjectName,
      webRequirements
    );
    
    setCurrentWorkflow(workflow);
    await executeWithStream(workflow);
  };
  
  const handleText2Backend = async () => {
    if (!backendProjectName || !backendRequirements) {
      alert('Please provide both project name and requirements');
      return;
    }
    
    const workflow = createText2BackendWorkflow(
      Date.now().toString(),
      backendProjectName,
      backendRequirements
    );
    
    setCurrentWorkflow(workflow);
    await executeWithStream(workflow);
  };
  
  const copyToClipboard = (text: string) => {
    navigator.clipboard.writeText(text);
  };
  
  return (
    <div className="min-h-screen bg-gradient-to-br from-slate-50 via-blue-50 to-indigo-50 dark:from-slate-950 dark:via-slate-900 dark:to-indigo-950 p-6">
      <div className="max-w-7xl mx-auto space-y-6">
        {/* Header */}
        <motion.div
          initial={{ opacity: 0, y: -20 }}
          animate={{ opacity: 1, y: 0 }}
          className="text-center space-y-4"
        >
          <div className="flex items-center justify-center gap-3">
            <motion.div
              animate={{ rotate: 360 }}
              transition={{ duration: 3, repeat: Infinity, ease: 'linear' }}
            >
              <Sparkles className="w-10 h-10 text-indigo-600 dark:text-indigo-400" />
            </motion.div>
            <h1 className="text-5xl font-bold bg-clip-text text-transparent bg-gradient-to-r from-indigo-600 to-purple-600 dark:from-indigo-400 dark:to-purple-400">
              DeepCode
            </h1>
          </div>
          <p className="text-xl text-muted-foreground max-w-3xl mx-auto">
            Open Agentic Coding - Transform ideas into production-ready code
          </p>
          <div className="flex items-center justify-center gap-6 text-sm text-muted-foreground">
            <div className="flex items-center gap-2">
              <FileText className="w-4 h-4 text-red-500" />
              <span>Paper2Code</span>
            </div>
            <div className="flex items-center gap-2">
              <Globe className="w-4 h-4 text-green-500" />
              <span>Text2Web</span>
            </div>
            <div className="flex items-center gap-2">
              <Server className="w-4 h-4 text-blue-500" />
              <span>Text2Backend</span>
            </div>
          </div>
        </motion.div>
        
        {/* Main Interface */}
        <div className="grid lg:grid-cols-2 gap-6">
          {/* Input Panel */}
          <motion.div
            initial={{ opacity: 0, x: -20 }}
            animate={{ opacity: 1, x: 0 }}
            transition={{ delay: 0.1 }}
          >
            <Card className="h-full">
              <CardHeader>
                <CardTitle className="flex items-center gap-2">
                  <Code2 className="w-5 h-5 text-indigo-600" />
                  Code Generation
                </CardTitle>
                <CardDescription>
                  Choose your workflow and provide input specifications
                </CardDescription>
              </CardHeader>
              <CardContent>
                <Tabs value={activeTab} onValueChange={(v) => setActiveTab(v as any)}>
                  <TabsList className="grid w-full grid-cols-3">
                    <TabsTrigger value="paper2code" className="flex items-center gap-2">
                      <FileText className="w-4 h-4" />
                      Paper2Code
                    </TabsTrigger>
                    <TabsTrigger value="text2web" className="flex items-center gap-2">
                      <Globe className="w-4 h-4" />
                      Text2Web
                    </TabsTrigger>
                    <TabsTrigger value="text2backend" className="flex items-center gap-2">
                      <Server className="w-4 h-4" />
                      Text2Backend
                    </TabsTrigger>
                  </TabsList>
                  
                  {/* Paper2Code Tab */}
                  <TabsContent value="paper2code" className="space-y-4">
                    <div className="space-y-2">
                      <Label htmlFor="paper-title">Paper Title</Label>
                      <Input
                        id="paper-title"
                        placeholder="e.g., Attention Is All You Need"
                        value={paperTitle}
                        onChange={(e) => setPaperTitle(e.target.value)}
                        disabled={isStreaming}
                      />
                    </div>
                    <div className="space-y-2">
                      <Label htmlFor="paper-content">Paper Content / Abstract</Label>
                      <Textarea
                        id="paper-content"
                        placeholder="Paste the paper abstract or key sections here..."
                        value={paperContent}
                        onChange={(e) => setPaperContent(e.target.value)}
                        rows={12}
                        disabled={isStreaming}
                        className="font-mono text-sm"
                      />
                      <p className="text-xs text-muted-foreground">
                        Tip: Include abstract, methodology, and key algorithms for best results
                      </p>
                    </div>
                    <Button
                      onClick={handlePaper2Code}
                      disabled={isStreaming || !paperTitle || !paperContent}
                      className="w-full"
                      size="lg"
                    >
                      {isStreaming ? (
                        <>
                          <Loader2 className="w-4 h-4 mr-2 animate-spin" />
                          Generating Code...
                        </>
                      ) : (
                        <>
                          <Play className="w-4 h-4 mr-2" />
                          Generate Implementation
                        </>
                      )}
                    </Button>
                  </TabsContent>
                  
                  {/* Text2Web Tab */}
                  <TabsContent value="text2web" className="space-y-4">
                    <div className="space-y-2">
                      <Label htmlFor="web-project">Project Name</Label>
                      <Input
                        id="web-project"
                        placeholder="e.g., E-commerce Dashboard"
                        value={webProjectName}
                        onChange={(e) => setWebProjectName(e.target.value)}
                        disabled={isStreaming}
                      />
                    </div>
                    <div className="space-y-2">
                      <Label htmlFor="web-requirements">Requirements</Label>
                      <Textarea
                        id="web-requirements"
                        placeholder="Describe the web application you want to build...\n\nExample: Create a modern e-commerce admin dashboard with product management, order tracking, real-time analytics charts, and user management. Use React, TypeScript, and Tailwind CSS. Include responsive design and dark mode."
                        value={webRequirements}
                        onChange={(e) => setWebRequirements(e.target.value)}
                        rows={12}
                        disabled={isStreaming}
                      />
                      <p className="text-xs text-muted-foreground">
                        Tip: Be specific about UI components, interactions, and styling preferences
                      </p>
                    </div>
                    <Button
                      onClick={handleText2Web}
                      disabled={isStreaming || !webProjectName || !webRequirements}
                      className="w-full"
                      size="lg"
                    >
                      {isStreaming ? (
                        <>
                          <Loader2 className="w-4 h-4 mr-2 animate-spin" />
                          Generating Frontend...
                        </>
                      ) : (
                        <>
                          <Play className="w-4 h-4 mr-2" />
                          Generate Frontend
                        </>
                      )}
                    </Button>
                  </TabsContent>
                  
                  {/* Text2Backend Tab */}
                  <TabsContent value="text2backend" className="space-y-4">
                    <div className="space-y-2">
                      <Label htmlFor="backend-project">Project Name</Label>
                      <Input
                        id="backend-project"
                        placeholder="e.g., Task Management API"
                        value={backendProjectName}
                        onChange={(e) => setBackendProjectName(e.target.value)}
                        disabled={isStreaming}
                      />
                    </div>
                    <div className="space-y-2">
                      <Label htmlFor="backend-requirements">Requirements</Label>
                      <Textarea
                        id="backend-requirements"
                        placeholder="Describe the backend API you want to build...\n\nExample: Create a RESTful API for a task management system with user authentication, CRUD operations for tasks and projects, role-based access control, real-time notifications, and file uploads. Use Node.js, Express, PostgreSQL, and Prisma."
                        value={backendRequirements}
                        onChange={(e) => setBackendRequirements(e.target.value)}
                        rows={12}
                        disabled={isStreaming}
                      />
                      <p className="text-xs text-muted-foreground">
                        Tip: Include API endpoints, data models, authentication, and integrations
                      </p>
                    </div>
                    <Button
                      onClick={handleText2Backend}
                      disabled={isStreaming || !backendProjectName || !backendRequirements}
                      className="w-full"
                      size="lg"
                    >
                      {isStreaming ? (
                        <>
                          <Loader2 className="w-4 h-4 mr-2 animate-spin" />
                          Generating Backend...
                        </>
                      ) : (
                        <>
                          <Play className="w-4 h-4 mr-2" />
                          Generate Backend
                        </>
                      )}
                    </Button>
                  </TabsContent>
                </Tabs>
              </CardContent>
            </Card>
          </motion.div>
          
          {/* Output Panel */}
          <motion.div
            initial={{ opacity: 0, x: 20 }}
            animate={{ opacity: 1, x: 0 }}
            transition={{ delay: 0.2 }}
            className="space-y-4"
          >
            {/* Workflow Monitor */}
            {currentWorkflow && isStreaming && (
              <WorkflowMonitor workflow={currentWorkflow} autoExecute={false} />
            )}
            
            {/* Result Display */}
            {!currentWorkflow && !isStreaming && (
              <Card className="h-full flex items-center justify-center">
                <CardContent className="text-center space-y-4 py-12">
                  <motion.div
                    animate={{ scale: [1, 1.1, 1] }}
                    transition={{ duration: 2, repeat: Infinity }}
                  >
                    <Sparkles className="w-16 h-16 mx-auto text-muted-foreground/30" />
                  </motion.div>
                  <div className="space-y-2">
                    <h3 className="text-lg font-semibold">Ready to Generate Code</h3>
                    <p className="text-sm text-muted-foreground max-w-sm mx-auto">
                      Fill in the form and click generate to start the agentic coding workflow
                    </p>
                  </div>
                </CardContent>
              </Card>
            )}
            
            {/* Final Result */}
            {result && (
              <Card>
                <CardHeader>
                  <CardTitle className="flex items-center gap-2">
                    <CheckCircle2 className="w-5 h-5 text-green-600" />
                    Generated Code
                  </CardTitle>
                </CardHeader>
                <CardContent>
                  <div className="space-y-3">
                    <pre className="bg-slate-100 dark:bg-slate-900 p-4 rounded-lg overflow-auto max-h-96 text-sm font-mono">
                      {JSON.stringify(result, null, 2)}
                    </pre>
                    <div className="flex gap-2">
                      <Button
                        variant="outline"
                        size="sm"
                        onClick={() => copyToClipboard(JSON.stringify(result, null, 2))}
                      >
                        <Copy className="w-4 h-4 mr-2" />
                        Copy Code
                      </Button>
                      <Button
                        variant="outline"
                        size="sm"
                        onClick={() => {
                          const blob = new Blob([JSON.stringify(result, null, 2)], { type: 'application/json' });
                          const url = URL.createObjectURL(blob);
                          const a = document.createElement('a');
                          a.href = url;
                          a.download = `deepcode-${activeTab}-${Date.now()}.json`;
                          a.click();
                        }}
                      >
                        <Download className="w-4 h-4 mr-2" />
                        Download
                      </Button>
                    </div>
                  </div>
                </CardContent>
              </Card>
            )}
            
            {/* Error Display */}
            {error && (
              <Card className="border-red-200 dark:border-red-900">
                <CardHeader>
                  <CardTitle className="flex items-center gap-2 text-red-600 dark:text-red-400">
                    <AlertCircle className="w-5 h-5" />
                    Error
                  </CardTitle>
                </CardHeader>
                <CardContent>
                  <p className="text-sm">{error}</p>
                </CardContent>
              </Card>
            )}
          </motion.div>
        </div>
        
        {/* Features Section */}
        <motion.div
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ delay: 0.3 }}
          className="grid md:grid-cols-3 gap-4"
        >
          <Card>
            <CardHeader>
              <CardTitle className="flex items-center gap-2 text-base">
                <FileText className="w-5 h-5 text-red-500" />
                Paper2Code
              </CardTitle>
            </CardHeader>
            <CardContent className="text-sm text-muted-foreground">
              <ul className="space-y-1 list-disc list-inside">
                <li>Algorithm extraction from papers</li>
                <li>Mathematical formula implementation</li>
                <li>Complete code with tests</li>
                <li>Documentation generation</li>
              </ul>
            </CardContent>
          </Card>
          
          <Card>
            <CardHeader>
              <CardTitle className="flex items-center gap-2 text-base">
                <Globe className="w-5 h-5 text-green-500" />
                Text2Web
              </CardTitle>
            </CardHeader>
            <CardContent className="text-sm text-muted-foreground">
              <ul className="space-y-1 list-disc list-inside">
                <li>React/Next.js components</li>
                <li>Responsive Tailwind CSS</li>
                <li>Accessibility (WCAG)</li>
                <li>State management</li>
              </ul>
            </CardContent>
          </Card>
          
          <Card>
            <CardHeader>
              <CardTitle className="flex items-center gap-2 text-base">
                <Server className="w-5 h-5 text-blue-500" />
                Text2Backend
              </CardTitle>
            </CardHeader>
            <CardContent className="text-sm text-muted-foreground">
              <ul className="space-y-1 list-disc list-inside">
                <li>REST API endpoints</li>
                <li>Database schema & migrations</li>
                <li>Authentication & security</li>
                <li>API documentation</li>
              </ul>
            </CardContent>
          </Card>
        </motion.div>
      </div>
    </div>
  );
}
