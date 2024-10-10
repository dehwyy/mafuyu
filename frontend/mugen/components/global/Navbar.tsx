'use client'

import { useCallback, useState } from 'react'
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
import { useAtom } from 'jotai'

import { Dev, TransformTranslate } from '@/lib/const'
import { NavbarAtom } from '@/lib/store/global'
import { IconChevronUp } from '../icons/ChevronUp'

export default function Navbar() {
  // with localstorage
  const [{ isExpanded }, setStore] = useAtom(NavbarAtom)

  const toggleNavbar = useCallback(() => {
    setStore((v) => ({ ...v, isExpanded: !v.isExpanded }))
  }, [])

  return (
    <NextUINavbar
      shouldHideOnScroll
      isBordered
      maxWidth="full"
      className={clsx(
        isExpanded || `!-translate-y-[${TransformTranslate.NavbarHidePx}] -mt-[${TransformTranslate.NavbarHidePx}]`,
        'transition-all'
      )}
    >
      <div className="absolute left-1/2 -translate-x-1/2 -bottom-3.5">
        <Button
          onClick={toggleNavbar}
          isIconOnly
          disableAnimation
          className="h-[48px] px-10 items-end bg-transparent focus-visible:outline-none"
        >
          <IconChevronUp className={clsx(isExpanded || 'rotate-180', 'transition-all stroke-default-300 min-w-[16px]')} />
        </Button>
      </div>
      <NavbarBrand className={clsx(isExpanded || '-translate-y-4', 'transition-all')}>
        <p className="font-bold text-xl">Mugen</p>
      </NavbarBrand>
      <NavbarContent
        justify="end"
        className={clsx(isExpanded || '-translate-y-4', 'transition-all')}
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
