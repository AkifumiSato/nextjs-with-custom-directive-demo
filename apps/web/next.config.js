/** @type {import('next').NextConfig} */
const nextConfig = {
  experimental: {
    swcPlugins: [["../../packages/swc-plugin-use-debug", {}]],
  },
};

export default nextConfig;
