import {
  defineConfig,
  presetIcons,
  presetUno,
  transformerDirectives,
} from 'unocss'

export default defineConfig({
  presets: [
    presetUno(),
    presetIcons({
      scale: 1.1,
      collections: {
        carbon: () =>
          import('@iconify-json/carbon/icons.json').then((i) => i.default),
      },
    }),
  ],
  transformers: [transformerDirectives()],
  theme: {
    colors: {
      base: '#0b0f14',
      surface: {
        1: '#121821',
        2: '#1a2230',
      },
      border: '#243044',
      main: '#e8edf5',
      muted: '#8b98ab',
      primary: '#3b82f6',
      rise: '#ef4444',
      fall: '#22c55e',
      flat: '#94a3b8',
      warning: '#f59e0b',
      positive: '#10b981',
    },
  },
  shortcuts: {
    'text-muted': 'text-muted',
  },
})
