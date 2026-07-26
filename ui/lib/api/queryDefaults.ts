type QueryOptions = Record<string, unknown>

export const withQueryDefaults = <T extends QueryOptions>(options: T) => ({
  gcTime: 5 * 60 * 1000,
  retry: 1,
  ...options,
})
