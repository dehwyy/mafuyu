import { useState } from 'react';
import { Button, Card, CardBody, Dropdown, DropdownItem, DropdownMenu, DropdownTrigger, User } from '@nextui-org/react';
import clsx from 'clsx';



import { IconChevronUp } from '@/components/icons/ChevronUp'
import { IconMessages } from '@/components/icons/Messages'
import { IconSettings } from '@/components/icons/Settings'
import { IconVolume } from '@/components/icons/Volume'
import { Dev } from '@/lib/const'

interface CurrentServerProps {
  className?: string
}

export function CurrentServer({ className }: CurrentServerProps) {
  const [isOpen, setIsOpen] = useState(false)
  return (
    <section className={clsx('flex-1 h-full flex flex-col gap-y-4 relative font-sans select-none', className)}>
      <Dropdown
        onOpenChange={setIsOpen}
        disableAnimation
      >
        <DropdownTrigger>
          <Button variant="light">
            <span className="text-[16px] font-semibold">Current server</span>
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

      <div className="px-3">
        <div className="h-[34px]">
          <div className="flex gap-x-1 items-center">
            <IconMessages className="h-5 min-w-5" />
            <p className="bold text-[16px] font-medium overflow-hidden text-ellipsis whitespace-nowrap">Example of text chat.12313112</p>
            <IconSettings className="h-5 min-w-5" />
          </div>
        </div>
        <div className="h-[34px]">
          <div className="flex gap-x-1 items-center">
            <IconVolume className="w-5 h-5" />
            <p className="bold text-[16px] font-medium overflow-hidden text-ellipsis whitespace-nowrap">Osu! enjoyers</p>
            <IconSettings className="h-5 w-5 ml-auto" />
          </div>
          <div className="items-start mt-2 ml-6 flex flex-col gap-y-3">
            {new Array(3).fill({}).map((_, i) => (
              <User
                classNames={{
                  name: ['text-[14px] font-medium']
                }}
                name="dehwyy"
                avatarProps={{ src: Dev.Img2, className: 'w-6 h-6' }}
              />
            ))}
          </div>
        </div>
      </div>
    </section>
  )
}
