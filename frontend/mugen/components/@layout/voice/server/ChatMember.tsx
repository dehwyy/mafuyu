import { User } from '@nextui-org/react'
import clsx from 'clsx'

import { Dev } from '@/lib/const'
import { UserCardPopover } from '../UserCardPopover'

export interface ChatMemberProps {
  username: string
  userImage?: string
}

export function ChatMember(props: ChatMemberProps) {
  return (
    <UserCardPopover
      username={props.username}
      userImage={props.userImage}
      popoverProps={{ placement: 'right-start' }}
      classNameOpen="bg-default-100"
      className="hover:bg-default-50"
    >
      <User
        className="w-full justify-start pl-3 py-1.5"
        classNames={{
          name: ['text-[14px] font-medium']
        }}
        name={props.username}
        avatarProps={{ src: props.userImage || Dev.Img2, className: 'w-6 h-6' }}
      />
    </UserCardPopover>
  )
}
