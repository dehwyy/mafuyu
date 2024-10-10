import { useMemo } from 'react'
import { Avatar, Button, Card, CardBody, ScrollShadow } from '@nextui-org/react'
import clsx from 'clsx'
import { useAtomValue } from 'jotai'

import { Dev, TransformTranslate } from '@/lib/const'
import { NavbarAtom } from '@/lib/store/global'

export default function ChatList() {
  const navbar = useAtomValue(NavbarAtom)
  const h = useMemo(() => {
    return 693 + +(navbar.isExpanded ? 0 : TransformTranslate.NavbarHide) + 'px'
  }, [navbar.isExpanded])

  return (
    <ScrollShadow
      style={{ maxHeight: h }}
      className="w-[250px] my-3 pr-1 transition-all"
    >
      <Card className={clsx('')}>
        <CardBody className="flex flex-col gap-y-2 !px-1 overflow-y-auto">
          {new Array(20).fill(0).map((_, i) => (
            <ChatListItem
              key={i}
              unreadMessages={i}
              chatSection={i === 5 ? 'ChatSectionId' : undefined}
              isFocused={i === 2}
            />
          ))}
        </CardBody>
      </Card>
    </ScrollShadow>
  )
}

function ChatListItem(props: { isFocused: boolean; unreadMessages: number; chatSection?: string }) {
  return (
    <Button
      className={clsx(props.isFocused ? '' : '', 'w-full focus-visible:!outline-none min-h-[50px] !pl-2 !pr-3 !py-8')}
      variant={props.isFocused ? 'shadow' : 'light'}
      color="default"
      radius="sm"
    >
      <div className="flex gap-x-2 w-full items-center">
        <Avatar src={Dev.Img} />
        <div className="flex flex-col items-start">
          <p className="font-semibold">Chat name</p>
          {props.chatSection && <p className="text-foreground/65 text-[11px] leading-4 ">"{props.chatSection}"</p>}
          <p className="text-foreground/80 text-[12px] leading-4">Last message</p>
        </div>
        <div className="ml-auto flex flex-col items-end gap-y-1 self-start">
          <p className="text-[10px] text-foreground/70">20:31</p>

          {props.unreadMessages > 0 && (
            <div className="grid place-items-center bg-content4 rounded-full  h-[22px] w-[22px]">
              <p className="text-[10px] ">{props.unreadMessages}</p>
            </div>
          )}
        </div>
      </div>
    </Button>
  )
}
