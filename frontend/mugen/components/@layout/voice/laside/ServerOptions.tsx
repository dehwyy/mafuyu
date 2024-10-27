'use client'

import { useMemo, useState } from 'react'
import { Button, Divider, Dropdown, DropdownItem, DropdownMenu, DropdownTrigger } from '@nextui-org/react'
import clsx from 'clsx'

import { IconChevronUp } from '@/components/icons/ChevronUp'

// TODO
const getServerOptions = (role: any) => {
  return [
    [{ name: 'Invite People', color: 'text-secondary-500' }, { name: 'Server Settings' }],
    [{ name: 'Edit Server Profile' }, { name: 'Notification Settings' }],
    [{ name: 'Leave Server', color: 'text-red-400' }]
  ]
}

interface ServerOptionsProps {
  serverName: string
  userRole: any
}

export function ServerOptions(props: ServerOptionsProps) {
  const [isOpen, setIsOpen] = useState(false)
  const options = useMemo(() => getServerOptions(props.userRole), [props.userRole])

  return (
    <Dropdown onOpenChange={setIsOpen} disableAnimation>
      <DropdownTrigger>
        <Button className={clsx(isOpen && 'bg-default-100')} variant="light">
          <span className="text-[16px] font-semibold">{props.serverName}</span>
          <div className={clsx(isOpen ? 'rotate-180' : 'rotate-0', 'transition-all duration-300 ease-in-out')}>
            <IconChevronUp />
          </div>
        </Button>
      </DropdownTrigger>
      <DropdownMenu>
        {options.map((option, i) => (
          <>
            {option.map((item, j) => (
              <DropdownItem
                color={'secondary'}
                className={clsx(item.color)}
                showDivider={i !== options.length - 1 && j === option.length - 1}
                key={item.name}
              >
                {item.name}
              </DropdownItem>
            ))}
          </>
        ))}
      </DropdownMenu>
    </Dropdown>
  )
}
