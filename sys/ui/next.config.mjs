/** @type {import('next').NextConfig} */
const nextConfig = {
    async rewrites ()
    {
        // Proxy UI calls like fetch('/api/v1/...') to the Rust API server.
        // Override if needed, e.g. NOA_API_BASE_URL=http://127.0.0.1:3001
        const apiBaseUrl = process.env.NOA_API_BASE_URL || 'http://127.0.0.1:3001';

        return [
            { source: '/health', destination: `${ apiBaseUrl }/health` },
            { source: '/api/:path*', destination: `${ apiBaseUrl }/api/:path*` },
        ];
    },
};

export default nextConfig;
