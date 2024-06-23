import { join } from 'path'
import { skeleton } from '@skeletonlabs/tw-plugin'
// @ts-ignore
import forms from '@tailwindcss/forms'
// @ts-ignore
import typography from '@tailwindcss/typography'
import type { CustomThemeConfig } from '@skeletonlabs/tw-plugin'
import type { Config } from 'tailwindcss'

const darkAgeTheme: CustomThemeConfig = {
  name: 'darkest',
  properties: {
    // =~= Theme Properties =~=
    '--theme-font-family-base': `'Quicksand', sans-serif`,
    '--theme-font-family-heading': `'Quicksand', sans-serif`,
    '--theme-font-color-base': '0 0 0',
    '--theme-font-color-dark': '255 255 255',
    '--theme-rounded-base': '9999px',
    '--theme-rounded-container': '16px',
    '--theme-border-base': '4px',
    '--on-primary': '0 0 0',
    '--on-secondary': '255 255 255',
    '--on-tertiary': '255 255 255',
    '--on-success': '0 0 0',
    '--on-warning': '0 0 0',
    '--on-error': '255 255 255',
    '--on-surface': '255 255 255',
    // =~= Theme Colors  =~=
    // primary | #ffffff
    '--color-primary-50': '255 255 255', // #ffffff
    '--color-primary-100': '255 255 255', // #ffffff
    '--color-primary-200': '255 255 255', // #ffffff
    '--color-primary-300': '255 255 255', // #ffffff
    '--color-primary-400': '255 255 255', // #ffffff
    '--color-primary-500': '255 255 255', // #ffffff
    '--color-primary-600': '230 230 230', // #e6e6e6
    '--color-primary-700': '191 191 191', // #bfbfbf
    '--color-primary-800': '153 153 153', // #999999
    '--color-primary-900': '125 125 125', // #7d7d7d
    // secondary | #d12e3e
    '--color-secondary-50': '227 237 243',
    '--color-secondary-100': '218 231 239',
    '--color-secondary-200': '209 225 235',
    '--color-secondary-300': '181 206 223',
    '--color-secondary-400': '126 170 199',
    '--color-secondary-500': '70 133 175',
    '--color-secondary-600': '63 120 158',
    '--color-secondary-700': '53 100 131',
    '--color-secondary-800': '125 28 37', // #7d1c25
    '--color-secondary-900': '102 23 30', // #66171e
    // tertiary | #1c0de7
    '--color-tertiary-50': '221 219 251', // #dddbfb
    '--color-tertiary-100': '210 207 250', // #d2cffa
    '--color-tertiary-200': '198 195 249', // #c6c3f9
    '--color-tertiary-300': '164 158 245', // #a49ef5
    '--color-tertiary-400': '96 86 238', // #6056ee
    '--color-tertiary-500': '28 13 231', // #1c0de7
    '--color-tertiary-600': '25 12 208', // #190cd0
    '--color-tertiary-700': '21 10 173', // #150aad
    '--color-tertiary-800': '17 8 139', // #11088b
    '--color-tertiary-900': '14 6 113', // #0e0671
    // success | #25ad34
    '--color-success-50': '222 243 225', // #def3e1
    '--color-success-100': '211 239 214', // #d3efd6
    '--color-success-200': '201 235 204', // #c9ebcc
    '--color-success-300': '168 222 174', // #a8deae
    '--color-success-400': '102 198 113', // #66c671
    '--color-success-500': '37 173 52', // #25ad34
    '--color-success-600': '33 156 47', // #219c2f
    '--color-success-700': '28 130 39', // #1c8227
    '--color-success-800': '22 104 31', // #16681f
    '--color-success-900': '18 85 25', // #125519
    // warning | #EAB308
    '--color-warning-50': '252 244 218', // #fcf4da
    '--color-warning-100': '251 240 206', // #fbf0ce
    '--color-warning-200': '250 236 193', // #faecc1
    '--color-warning-300': '247 225 156', // #f7e19c
    '--color-warning-400': '240 202 82', // #f0ca52
    '--color-warning-500': '234 179 8', // #EAB308
    '--color-warning-600': '211 161 7', // #d3a107
    '--color-warning-700': '176 134 6', // #b08606
    '--color-warning-800': '140 107 5', // #8c6b05
    '--color-warning-900': '115 88 4', // #735804
    // error | #d21919
    '--color-error-50': '248 221 221', // #f8dddd
    '--color-error-100': '246 209 209', // #f6d1d1
    '--color-error-200': '244 198 198', // #f4c6c6
    '--color-error-300': '237 163 163', // #eda3a3
    '--color-error-400': '224 94 94', // #e05e5e
    '--color-error-500': '210 25 25', // #d21919
    '--color-error-600': '189 23 23', // #bd1717
    '--color-error-700': '158 19 19', // #9e1313
    '--color-error-800': '126 15 15', // #7e0f0f
    '--color-error-900': '103 12 12', // #670c0c
    // surface | #111111
    '--color-surface-50': '219 219 219', // #dbdbdb
    '--color-surface-100': '207 207 207', // #cfcfcf
    '--color-surface-200': '196 196 196', // #c4c4c4
    '--color-surface-300': '160 160 160', // #a0a0a0
    '--color-surface-400': '88 88 88', // #585858
    '--color-surface-500': '17 17 17', // #111111
    '--color-surface-600': '15 15 15', // #0f0f0f
    '--color-surface-700': '13 13 13', // #0d0d0d
    '--color-surface-800': '10 10 10', // #0a0a0a
    '--color-surface-900': '8 8 8' // #080808
  }
}
export default {
  darkMode: 'class',
  content: [
    './src/**/*.{html,js,svelte,ts}',
    join(
      require.resolve('@skeletonlabs/skeleton'),
      '../**/*.{html,js,svelte,ts}'
    )
  ],
  theme: {
    extend: {}
  },
  plugins: [
    forms,
    typography,
    skeleton({
      themes: {
        custom: [darkAgeTheme],
        preset: [
          {
            name: 'wintry',
            enhancements: true
          },
          {
            name: 'modern',
            enhancements: true
          },
          {
            name: 'crimson',
            enhancements: true
          }
        ]
      }
    })
  ]
} satisfies Config
