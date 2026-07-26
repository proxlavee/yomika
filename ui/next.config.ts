import path from 'node:path'
import { fileURLToPath } from 'node:url'

import { withSentryConfig } from '@sentry/nextjs'
import type { NextConfig } from 'next'

const repositoryRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..')
const isDevelopment = process.env.NODE_ENV === 'development'

const nextConfig: NextConfig = {
  reactCompiler: true,
  devIndicators: false,
  output: 'export',
  // Next's dev server otherwise gzip-compresses proxied responses; zlib
  // buffers small `text/event-stream` chunks until its internal window
  // fills, which never happens for low-volume SSE → the UI sees the
  // connection open but no frames arrive. Browsers can't opt out via
  // `Accept-Encoding: identity` (it's a forbidden request header), so
  // this has to be a server-side switch. Safe in prod: `output: 'export'`
  // means the Rust backend serves the static UI directly — Next's
  // compression layer is only in the picture during `next dev`.
  compress: false,
  images: {
    unoptimized: true,
  },
  // The repository may live below unrelated package-lock files. Pinning the
  // root prevents Turbopack from treating an ancestor user directory as the
  // workspace and scanning/building outside this checkout.
  turbopack: {
    root: repositoryRoot,
  },
  ...(isDevelopment
    ? {
        experimental: {
          proxyClientMaxBodySize: '1gb' as const,
          proxyTimeout: 300000,
        },
        async rewrites() {
          return [
            {
              source: '/api/v1/:path*',
              destination: 'http://127.0.0.1:4000/api/v1/:path*',
            },
          ]
        },
      }
    : {}),
}

const sentryOrg = process.env.SENTRY_ORG
const sentryProject = process.env.SENTRY_PROJECT

export default sentryOrg && sentryProject
  ? withSentryConfig(nextConfig, {
      org: sentryOrg,
      project: sentryProject,
      silent: !process.env.CI,
    })
  : nextConfig
