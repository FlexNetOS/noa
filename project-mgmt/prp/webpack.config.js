import path from 'path';
import { fileURLToPath } from 'url';
import HtmlWebpackPlugin from 'html-webpack-plugin';
import CopyWebpackPlugin from 'copy-webpack-plugin';
import fs from 'fs-extra';
import { glob } from 'glob';

// Analytics and Performance Monitoring Plugin
class AnalyticsPlugin {
  apply(compiler) {
    compiler.hooks.compilation.tap('AnalyticsPlugin', (compilation) => {
      compilation.hooks.processAssets.tap(
        {
          name: 'AnalyticsPlugin',
          stage: compilation.PROCESS_ASSETS_STAGE_OPTIMIZE_INLINE
        },
        () => {
          const analyticsCode = `
<!-- Analytics & Monitoring -->
<script>
  // Performance monitoring
  if ('performance' in window && 'measure' in window.performance) {
    window.addEventListener('load', function() {
      setTimeout(function() {
        const perfData = performance.getEntriesByType('navigation')[0];
        const loadTime = perfData.loadEventEnd - perfData.loadEventStart;
        console.log('Page load time:', loadTime + 'ms');
      }, 0);
    });
  }

  // Error tracking
  window.addEventListener('error', function(e) {
    const errorData = {
      message: e.message,
      filename: e.filename,
      lineno: e.lineno,
      colno: e.colno,
      url: window.location.href,
      timestamp: new Date().toISOString()
    };
    console.error('JavaScript Error:', errorData);
  });

  // Custom event tracking
  window.trackEvent = function(eventName, properties = {}) {
    console.log('Event tracked:', eventName, properties);
  };

  // Track external link clicks
  document.addEventListener('click', function(e) {
    const target = e.target.closest('a[href^="http"]:not([href*="prp.theedgestory.org"])');
    if (target) {
      trackEvent('external_link', {
        url: target.href,
        text: target.textContent?.trim()
      });
    }
  });

  console.log('📊 Analytics and monitoring initialized');
</script>

<!-- Plausible Analytics -->
<script defer data-domain="prp.theedgestory.org"
        src="https://plausible.io/js/script.js"
        data-outbound-links="true"
        data-tag-events="true">
</script>

<!-- Performance Observer API -->
<script>
if ('PerformanceObserver' in window) {
  // Track Core Web Vitals
  const observer = new PerformanceObserver((list) => {
    for (const entry of list.getEntries()) {
      switch (entry.entryType) {
        case 'largest-contentful-paint':
          trackEvent('lcp', { value: Math.round(entry.startTime) });
          break;
        case 'first-input':
          trackEvent('fid', { value: Math.round(entry.processingStart - entry.startTime) });
          break;
        case 'layout-shift':
          if (!entry.hadRecentInput) {
            trackEvent('cls', { value: Math.round(entry.value * 1000) / 1000 });
          }
          break;
      }
    }
  });

  observer.observe({ entryTypes: ['largest-contentful-paint', 'first-input', 'layout-shift'] });
}
</script>
          `;

          // Add analytics code to HTML files
          Object.keys(compilation.assets).forEach(filename => {
            if (filename.endsWith('.html')) {
              const source = compilation.assets[filename].source();
              const modifiedSource = source.replace('</head>', analyticsCode + '</head>');
              compilation.assets[filename] = {
                source: () => modifiedSource,
                size: () => modifiedSource.length
              };
            }
          });
        }
      );
    });
  }
}

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

export default (env, argv) => {
  const isProduction = argv.mode === 'production';

  return {
    mode: isProduction ? 'production' : 'development',
    entry: {
      main: './src/docs/index.js',
      // Generate entries for each MDX file
      ...generateMDXEntries()
    },
    output: {
      path: path.resolve(__dirname, 'build'),
      filename: isProduction ? '[name].[contenthash].js' : '[name].js',
      clean: true,
      publicPath: '/'
    },
    module: {
      rules: [
        {
          test: /\.mdx?$/,
          use: [
            {
              loader: 'babel-loader',
              options: {
                presets: ['@babel/preset-env', '@babel/preset-react']
              }
            },
            '@mdx-js/loader'
          ]
        },
        {
          test: /\.jsx?$/,
          exclude: /node_modules/,
          use: {
            loader: 'babel-loader',
            options: {
              presets: ['@babel/preset-env', '@babel/preset-react']
            }
          }
        },
        {
          test: /\.css$/,
          use: ['style-loader', 'css-loader']
        },
        {
          test: /\.(png|svg|jpg|jpeg|gif)$/i,
          type: 'asset/resource',
          generator: {
            filename: 'images/[name].[hash][ext]'
          }
        },
        {
          test: /\.(woff|woff2|eot|ttf|otf)$/i,
          type: 'asset/resource',
          generator: {
            filename: 'fonts/[name].[hash][ext]'
          }
        }
      ]
    },
    plugins: [
      // Analytics and monitoring
      new AnalyticsPlugin(),
      // Generate HTML files for each documentation page
      ...generateHTMLPlugins(),
      // Copy static assets
      new CopyWebpackPlugin({
        patterns: [
          { from: 'CNAME', to: '[name][ext]', noErrorOnMissing: true },
          { from: 'images', to: 'images', noErrorOnMissing: true },
          { from: 'docs/images', to: 'images/docs', noErrorOnMissing: true },
          { from: '*.png', to: '[name][ext]', noErrorOnMissing: true },
          { from: '*.jpg', to: '[name][ext]', noErrorOnMissing: true },
          { from: '*.ico', to: '[name][ext]', noErrorOnMissing: true }
        ]
      }),
      // Generate search index and sitemap
      new class SearchIndexPlugin {
        apply(compiler) {
          compiler.hooks.done.tap('SearchIndexPlugin', () => {
            generateSearchIndex();
            generateSitemap();
          });
        }
      }()
    ],
    resolve: {
      extensions: ['.js', '.jsx', '.mdx', '.md'],
      alias: {
        '@': path.resolve(__dirname, 'src')
      }
    },
    devtool: isProduction ? 'source-map' : 'eval-source-map',
    devServer: {
      static: {
        directory: path.join(__dirname, 'build'),
      },
      compress: true,
      port: 3000,
      open: true,
      historyApiFallback: true
    }
  };
};

function generateMDXEntries() {
  const entries = {};
  const mdxFiles = glob.sync('docs/**/*.mdx');

  mdxFiles.forEach(file => {
    const name = file
      .replace(/^docs\//, '')
      .replace(/\.mdx$/, '')
      .replace(/\//g, '-');
    entries[name] = `./${file}`;
  });

  return entries;
}

function generateHTMLPlugins() {
  const plugins = [];

  // Main landing page
  plugins.push(new HtmlWebpackPlugin({
    template: './index.html',
    filename: 'index.html',
    chunks: [],
    inject: false
  }));

  // Documentation pages
  const mdxFiles = glob.sync('docs/**/*.mdx');

  mdxFiles.forEach(file => {
    const outputPath = file
      .replace(/^docs\//, '')
      .replace(/\.mdx$/, '.html');

    const chunkName = file
      .replace(/^docs\//, '')
      .replace(/\.mdx$/, '')
      .replace(/\//g, '-');

    plugins.push(new HtmlWebpackPlugin({
      template: './src/docs/template.html',
      filename: outputPath,
      chunks: [chunkName, 'main'],
      templateParameters: {
        mdxPath: file,
        title: generateTitle(outputPath),
        breadcrumb: generateBreadcrumb(outputPath)
      }
    }));
  });

  // Generate HTML for regular markdown files
  const mdFiles = glob.sync('docs/**/*.md');

  mdFiles.forEach(file => {
    const outputPath = file
      .replace(/^docs\//, '')
      .replace(/\.md$/, '.html');

    plugins.push(new HtmlWebpackPlugin({
      template: './src/docs/markdown-template.html',
      filename: outputPath,
      templateParameters: {
        mdPath: file,
        title: generateTitle(outputPath),
        breadcrumb: generateBreadcrumb(outputPath)
      }
    }));
  });

  return plugins;
}

function generateTitle(outputPath) {
  const parts = outputPath.split('/');
  const fileName = parts[parts.length - 1].replace('.html', '');

  // Convert kebab-case to title case
  return fileName
    .split('-')
    .map(word => word.charAt(0).toUpperCase() + word.slice(1))
    .join(' ');
}

function generateBreadcrumb(outputPath) {
  const parts = outputPath.split('/');
  const breadcrumb = [
    { name: 'Home', url: '/' },
    { name: 'Documentation', url: '/docs/' }
  ];

  if (parts.length > 1) {
    let path = '/docs/';
    for (let i = 0; i < parts.length - 1; i++) {
      path += parts[i] + '/';
      breadcrumb.push({
        name: generateTitle(parts[i]),
        url: path
      });
    }
  }

  // Current page
  const currentPage = parts[parts.length - 1].replace('.html', '');
  breadcrumb.push({
    name: generateTitle(currentPage),
    url: null,
    active: true
  });

  return breadcrumb;
}

function generateSearchIndex() {
  const searchIndex = [];
  const buildDir = path.resolve(__dirname, 'build');

  // Index all HTML files
  const htmlFiles = glob.sync(path.join(buildDir, '**/*.html'));

  htmlFiles.forEach(file => {
    const content = fs.readFileSync(file, 'utf8');
    const relativePath = path.relative(buildDir, file);

    // Extract title and description
    const titleMatch = content.match(/<title>(.*?)<\/title>/);
    const h1Match = content.match(/<h1[^>]*>(.*?)<\/h1>/);
    const title = titleMatch?.[1] || h1Match?.[1] || 'Untitled';

    // Extract meta description
    const descMatch = content.match(/<meta[^>]*name=["']description["'][^>]*content=["']([^"']*)["']/);
    const description = descMatch?.[1] || '';

    // Strip HTML tags for content indexing
    const textContent = content
      .replace(/<script[^>]*>.*?<\/script>/gs, '')
      .replace(/<style[^>]*>.*?<\/style>/gs, '')
      .replace(/<[^>]*>/g, ' ')
      .replace(/\s+/g, ' ')
      .trim()
      .substring(0, 1000); // Limit content length

    searchIndex.push({
      title,
      description,
      content: textContent,
      url: '/' + relativePath,
      path: relativePath
    });
  });

  // Write search index
  fs.ensureDirSync(path.join(buildDir, 'assets'));
  fs.writeJSONSync(path.join(buildDir, 'assets', 'search-index.json'), searchIndex, { spaces: 2 });

  console.log(`✅ Generated search index with ${searchIndex.length} pages`);
}

function generateSitemap() {
  const buildDir = path.resolve(__dirname, 'build');
  const BASE_URL = 'https://prp.theedgestory.org';

  // Get all HTML files
  const htmlFiles = glob.sync(path.join(buildDir, '**/*.html'));

  let sitemap = `<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">`;

  // Add main page first
  sitemap += `
  <url>
    <loc>${BASE_URL}/</loc>
    <lastmod>${new Date().toISOString().split('T')[0]}</lastmod>
    <changefreq>weekly</changefreq>
    <priority>1.0</priority>
  </url>`;

  // Add all other pages
  htmlFiles.forEach(file => {
    const relativePath = path.relative(buildDir, file);
    const url = relativePath === 'index.html' ? null : `${BASE_URL}/${relativePath}`;

    if (url) {
      sitemap += `
  <url>
    <loc>${url}</loc>
    <lastmod>${new Date().toISOString().split('T')[0]}</lastmod>
    <changefreq>weekly</changefreq>
    <priority>0.8</priority>
  </url>`;
    }
  });

  sitemap += '\n</urlset>';

  // Write sitemap
  fs.writeFileSync(path.join(buildDir, 'sitemap.xml'), sitemap);
  console.log(`✅ Generated sitemap with ${htmlFiles.length} pages`);
}