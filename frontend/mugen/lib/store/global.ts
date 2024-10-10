import { atomWithStorage } from 'jotai/utils'

export const NavbarAtom = atomWithStorage('navbar', {
  isExpanded: true
})
