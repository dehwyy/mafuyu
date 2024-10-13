import { Fira_Sans, Fira_Code as FontMono, Open_Sans, Sacramento } from 'next/font/google'

export const fontSans = Open_Sans({
  weight: ['400', '500', '600', '700'],
  subsets: ['latin', 'cyrillic', 'cyrillic-ext'],
  variable: '--font-sans'
})

export const fontMono = FontMono({
  subsets: ['latin'],
  variable: '--font-mono'
})

export const fontSacramento = Sacramento({
  weight: '400',
  subsets: ['latin'],
  variable: '--font-sacramento'
})

export const fontCommon = Fira_Sans({
  weight: ['400', '500', '600', '700'],
  subsets: ['latin', 'cyrillic', 'cyrillic-ext'],
  variable: '--font-common'
})
