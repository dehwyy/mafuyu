import { Avatar } from '@nextui-org/react'

import { Dev } from '@/lib/const'

export interface RAsideUserProps {
  username: string
  userImage?: string
  activity?: string
}

export function RAsideUser(props: RAsideUserProps) {
  return (
    <div className="flex gap-x-3 items-center h-[44px] py-[1px]">
      <div>
        <Avatar className="h-8 w-8" src={props.userImage || Dev.Img} />
      </div>
      <div className="w-2/3">
        <p className="text-[16px] leading-5 text-violet-400">{props.username}</p>
        <p className="ellipsis text-[12px] leading-3 text-default-400 font-medium">{props.activity}</p>
      </div>
    </div>
  )
}
