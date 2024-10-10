'use client'

import { useState } from 'react'
import {
  Avatar,
  Button,
  Dropdown,
  DropdownItem,
  DropdownMenu,
  DropdownSection,
  DropdownTrigger,
  NavbarBrand,
  NavbarContent,
  NavbarItem,
  Navbar as NextUINavbar
} from '@nextui-org/react'
import clsx from 'clsx'

import { Dev } from '@/lib/const'
import { IconChevronUp } from '../icons/ChevronUp'

export default function Navbar() {
  // with localstorage
  const [isOpen, setOpen] = useState(true)

  return (
    <NextUINavbar
      shouldHideOnScroll
      isBordered
      maxWidth="full"
      className={clsx(isOpen || '!-translate-y-[50px] -mt-[50px]', 'transition-all')}
    >
      <div className="absolute left-1/2 -translate-x-1/2 -bottom-3.5">
        <Button
          onClick={() => setOpen((v) => !v)}
          isIconOnly
          disableAnimation
          className="h-[48px] px-10 items-end bg-transparent"
        >
          <IconChevronUp className={clsx(isOpen || 'rotate-180', 'transition-all stroke-default-300 min-w-[16px]')} />
        </Button>
      </div>
      <NavbarBrand className={clsx(isOpen || '-translate-y-4', 'transition-all')}>
        <p className="font-bold text-xl">Mugen</p>
      </NavbarBrand>
      <NavbarContent
        justify="end"
        className={clsx(isOpen || '-translate-y-4', 'transition-all')}
      >
        <NavbarItem>
          <Dropdown>
            <DropdownTrigger>
              <Avatar
                isBordered
                as="button"
                src={Dev.Img}
              />
            </DropdownTrigger>
            <DropdownMenu>
              {['Profile', 'Settings', 'Cookies', 'Sign out'].map((v, i) => (
                <DropdownItem
                  key="new"
                  href="/"
                  variant="faded"
                  className="py-2"
                >
                  {v}
                </DropdownItem>
              ))}
            </DropdownMenu>
          </Dropdown>
        </NavbarItem>
      </NavbarContent>
    </NextUINavbar>
  )
}
