import { User } from '@nextui-org/react'

import { Dev } from '@/lib/const'

export interface ChatMemberProps {
  username: string
  userImage?: string
}

export function ChatMember(props: ChatMemberProps) {
  return (
    <User
      className="w-full justify-start pl-3 py-1.5 hover:bg-default-50/50"
      classNames={{
        name: ['text-[14px] font-medium']
      }}
      name={props.username}
      avatarProps={{ src: props.userImage || Dev.Img2, className: 'w-6 h-6' }}
    />
  )
}
