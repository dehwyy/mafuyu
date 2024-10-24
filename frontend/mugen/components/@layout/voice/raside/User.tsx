import { useState } from 'react'
import {
  Avatar,
  Button,
  Card,
  CardBody,
  CardFooter,
  CardHeader,
  Chip,
  Image,
  Popover,
  PopoverContent,
  PopoverTrigger
} from '@nextui-org/react'
import clsx from 'clsx'

import { Dev } from '@/lib/const'
import { UserCard } from '../UserCard'

export interface RAsideUserProps {
  username: string
  userImage?: string
  activity?: string
}

export function RAsideUser(props: RAsideUserProps) {
  const [isOpen, setOpen] = useState(false)

  return (
    <Popover isOpen={isOpen} onOpenChange={setOpen} placement="left-start" offset={30}>
      <PopoverTrigger>
        <button
          aria-expanded="false"
          className={clsx(isOpen ? 'bg-default-100' : '', 'flex gap-x-3 items-center h-[44px] py-[1px] px-2 rounded-sm transition-all')}
        >
          <div>
            <Avatar className="h-8 w-8" src={props.userImage || Dev.Img} />
          </div>
          <div className="w-2/3">
            <p className="text-[16px] leading-5 text-violet-400">{props.username}</p>
            <p className="ellipsis text-[12px] leading-3 text-default-400 font-medium">{props.activity}</p>
          </div>
        </button>
      </PopoverTrigger>
      <PopoverContent className="p-0">
        <UserCard username={props.username} userImage={props.userImage} />
      </PopoverContent>
    </Popover>
  )
}
