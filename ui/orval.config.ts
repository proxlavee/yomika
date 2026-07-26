import { defineConfig } from 'orval'

export default defineConfig({
  yomika: {
    input: './openapi.json',
    output: {
      target: './lib/api',
      schemas: './lib/api/schemas',
      client: 'react-query',
      mode: 'tags-split',
      baseUrl: '/api/v1',
      mock: {
        generators: [{ type: 'msw' }],
      },
      override: {
        mock: {
          properties: {
            'login.status': 'pending',
          },
          schemas: {
            TextStyle: { properties: { textAlign: 'left' } },
          },
        },
        fetch: {
          includeHttpResponseReturnType: false,
        },
        mutator: {
          path: './lib/api/fetch.ts',
          name: 'fetchApi',
        },
        operations: {
          createPages: {
            formData: true,
          },
          addImageLayer: {
            formData: true,
          },
        },
        query: {
          queryOptions: {
            path: './lib/api/queryDefaults.ts',
            name: 'withQueryDefaults',
          },
        },
      },
    },
  },
})
