import { Avatar } from '@nextui-org/react'
import clsx from 'clsx'

import { Dev } from '@/lib/const'
import { UserCardPopover } from '../UserCardPopover'

export interface RAsideUserProps {
  username: string
  userImage?: string
  activity?: string
}

export function RAsideUser(props: RAsideUserProps) {
  return (
    <UserCardPopover
      username={props.username}
      userImage={props.userImage}
      popoverProps={{ placement: 'left-start', offset: 30 }}
      classNameOpen="bg-default-100"
      className="hover:bg-default-50"
    >
      <button className={'flex gap-x-3 items-center h-[44px] py-[1px] px-2 rounded-sm transition-all'}>
        <div>
          <Avatar className="h-8 w-8" src={props.userImage || Dev.Img} />
        </div>
        <div className="w-2/3">
          <p className="text-[16px] leading-5 text-violet-400">{props.username}</p>
          <p className="ellipsis text-[12px] leading-3 text-default-400 font-medium">{props.activity}</p>
        </div>
      </button>
    </UserCardPopover>
  )
}
