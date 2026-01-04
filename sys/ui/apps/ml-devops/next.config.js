const path = require( 'path' );

/** @type {import('next').Nextconfigs} */
const nextconfigs = {
  distDir: process.env.NEXT_DIST_DIR || '.next',
  output: process.env.NEXT_OUTPUT_MODE,
  experimental: {
    outputFileTracingRoot: path.join( __dirname, '../../' ),
  },
  async rewrites ()
  {
    const apiBase = process.env.NOA_API_URL || 'http://127.0.0.1:3001';
    return [
      {
        source: '/api/v1/:path*',
        destination: `${ apiBase }/api/v1/:path*`,
      },
    ];
  },
  eslint: {
    ignoreDuringBuilds: true,
  },
  typescript: {
    ignoreBuildErrors: false,
  },
  images: { unoptimized: true },
};

module.exports = nextconfigs;
