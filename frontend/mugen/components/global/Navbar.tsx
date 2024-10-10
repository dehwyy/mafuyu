'use client'

import { Avatar, NavbarBrand, NavbarContent, NavbarItem, Navbar as NextUINavbar } from '@nextui-org/react'

import { Dev } from '@/lib/const'

export default function Navbar() {
  return (
    <NextUINavbar
      shouldHideOnScroll
      isBordered
    >
      <NavbarBrand>
        <p className="font-bold text-inherit">Mugen</p>
      </NavbarBrand>
      <NavbarContent justify="end">
        <NavbarItem>
          <Avatar src={Dev.Img} />
        </NavbarItem>
      </NavbarContent>
    </NextUINavbar>
  )
}
