import { useState } from 'react'
import { Button } from '@nextui-org/button'
import { Avatar } from '@nextui-org/react'

import { Dev } from '@/lib/const'

interface ChatListItemProps {
  name: string
  avatarSrc?: string
  description?: string
  lastAction?: string
  lastActionTime: string

  isFocused?: boolean
  onClickAction?: () => void
  notificationsCount: number
}
interface ChatListProps {
  items: ChatListItemProps[]
}

export function ChatList({ items }: ChatListProps) {
  const [focused, setFocused] = useState(0)
  return (
    <section>
      {new Array(20).fill(0).map((_, i) => (
        <Item
          key={i}
          name="dehwyy"
          avatarSrc={Dev.Img}
          description="ИУ5"
          lastAction="#believer"
          lastActionTime="20:31"
          isFocused={focused === i}
          onClickAction={() => {
            setFocused(i)
          }}
          notificationsCount={i}
        />
      ))}
    </section>
  )
}

interface ChatListItemProps {
  name: string
  avatarSrc?: string
  description?: string
  lastAction?: string
  lastActionTime: string

  isFocused?: boolean
  onClickAction?: () => void
  notificationsCount: number
}

function Item(props: ChatListItemProps) {
  return (
    <Button
      className="w-full focus-visible:!outline-none min-h-[50px] !pl-2 !pr-3 !py-8"
      variant={props.isFocused ? 'shadow' : 'light'}
      color={props.isFocused ? 'secondary' : 'default'}
      radius="sm"
      onClick={props.onClickAction}
    >
      <div className="flex gap-x-2 w-full items-center">
        <Avatar src={props.avatarSrc || Dev.Img} />
        <div className="flex flex-col items-start">
          <p className="font-semibold">Chat name</p>
          {props.description && <p className="text-foreground/65 text-[11px] leading-4 ">"{props.description}"</p>}
          <p className="text-foreground/80 text-[12px] leading-4">{props.lastAction}</p>
        </div>
        <div className="ml-auto flex flex-col items-end gap-y-1 self-start">
          <p className="text-[10px] text-foreground/70">{props.lastActionTime}</p>

          {props.notificationsCount > 0 && (
            <div className="grid place-items-center bg-content4 rounded-full  h-[22px] w-[22px]">
              <p className="text-[10px] ">{props.notificationsCount}</p>
            </div>
          )}
        </div>
      </div>
    </Button>
  )
}
