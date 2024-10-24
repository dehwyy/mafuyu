import { useState } from 'react'
import { Popover, PopoverContent, PopoverTrigger, User } from '@nextui-org/react'
import clsx from 'clsx'

import { Dev } from '@/lib/const'
import { UserCard } from '../UserCard'

export interface ChatMemberProps {
  username: string
  userImage?: string
}

export function ChatMember(props: ChatMemberProps) {
  const [isOpen, setOpen] = useState(false)

  return (
    <Popover isOpen={isOpen} onOpenChange={setOpen} placement="right-start">
      <PopoverTrigger>
        <User
          aria-expanded="false"
          className={clsx(isOpen ? 'bg-default-100' : 'hover:bg-default-50/50', 'w-full justify-start pl-3 py-1.5  transition-all')}
          classNames={{
            name: ['text-[14px] font-medium']
          }}
          name={props.username}
          avatarProps={{ src: props.userImage || Dev.Img2, className: 'w-6 h-6' }}
        />
      </PopoverTrigger>
      <PopoverContent className="p-0">
        <UserCard username={props.username} userImage={props.userImage} />
      </PopoverContent>
    </Popover>
  )
}
