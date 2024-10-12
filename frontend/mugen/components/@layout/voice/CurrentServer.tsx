import { useState } from 'react'
import { Button, Card, CardBody, Dropdown, DropdownItem, DropdownMenu, DropdownTrigger, User } from '@nextui-org/react'
import clsx from 'clsx'

import { IconChevronUp } from '@/components/icons/ChevronUp'
import { Dev } from '@/lib/const'

interface CurrentServerProps {
  className?: string
}

export function CurrentServer({ className }: CurrentServerProps) {
  const [isOpen, setIsOpen] = useState(false)
  return (
    <section className={clsx('flex-1 h-full flex flex-col gap-y-4 items-start relative', className)}>
      <Dropdown onOpenChange={setIsOpen}>
        <DropdownTrigger>
          <Button variant="light">
            Current server
            <div className={clsx(isOpen ? 'rotate-180' : 'rotate-0', 'transition-all duration-300 ease-in-out')}>
              <IconChevronUp />
            </div>
          </Button>
        </DropdownTrigger>
        <DropdownMenu>
          <DropdownItem>Option 1 </DropdownItem>
          <DropdownItem>Option 2 </DropdownItem>
          <DropdownItem>Option 3 </DropdownItem>
        </DropdownMenu>
      </Dropdown>

      <div className="text-sm">
        <p className="semibold underline">-Channel name</p>
        <div className="items-start ml-2 mt-2 flex flex-col gap-y-3">
          {new Array(6).fill({}).map((_, i) => (
            <User
              className="text-[11px]"
              name="dehwyy"
              avatarProps={{ src: Dev.Img2, className: 'w-8 h-8' }}
            />
          ))}
        </div>
      </div>
    </section>
  )
}
