import { useState } from 'react'
import { Popover, PopoverContent, PopoverProps, PopoverTrigger } from '@nextui-org/react'
import clsx from 'clsx'

import { UserCard } from './UserCard'

interface UserCardPopoverProps {
  children: React.ReactNode
  popoverProps?: Partial<PopoverProps>

  // styling for `wrapper`
  className?: string
  classNameOpen?: string

  username: string
  userImage?: string
}

export function UserCardPopover(props: UserCardPopoverProps) {
  const [isOpen, setOpen] = useState(false)

  return (
    <Popover isOpen={isOpen} onOpenChange={setOpen} {...props.popoverProps}>
      <PopoverTrigger>
        <div aria-expanded="false" className={clsx(isOpen ? props.classNameOpen : props.className, 'transition-all rounded-sm')}>
          {props.children}
        </div>
      </PopoverTrigger>
      <PopoverContent className="p-0">
        <UserCard username={props.username} userImage={props.userImage} />
      </PopoverContent>
    </Popover>
  )
}
