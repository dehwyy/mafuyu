import { useMemo, useState } from 'react'
import { Avatar, Button, Card, CardBody, ScrollShadow } from '@nextui-org/react'
import clsx from 'clsx'
import { useAtomValue } from 'jotai'

import { Dev, TransformTranslate } from '@/lib/const'
import { NavbarAtom } from '@/lib/store/global'
import { MainLayoutManagementMenu } from '../aside/MainLayoutManagementMenu'
import { OverlaysManagementMenu } from '../aside/OverlaysManagementMenu'
import { UserShort } from '../aside/UserShort'

export default function ChatList() {
  const [focused, setFocused] = useState(0)

  return (
    <section className="flex flex-col gap-y-3 pt-3 w-[250px] ">
      <UserShort />
      <OverlaysManagementMenu />
      <ScrollShadow
        style={{ maxHeight: 'calc(100% - 70px)' }}
        className="mb-3 pr-1 transition-all"
      >
        <Card className={clsx('bg-default-100/50')}>
          <CardBody className="flex flex-col gap-y-2 !px-1 overflow-y-auto">
            {new Array(20).fill(0).map((_, i) => (
              <ChatListItem
                key={i}
                setFocused={() => setFocused(i)}
                unreadMessages={i}
                chatSection={i === 5 ? 'ChatSectionId' : undefined}
                isFocused={i === focused}
              />
            ))}
          </CardBody>
        </Card>
      </ScrollShadow>
    </section>
  )
}

function ChatListItem(props: { isFocused: boolean; setFocused: () => void; unreadMessages: number; chatSection?: string }) {
  return (
    <Button
      className={clsx(props.isFocused ? '' : '', 'w-full focus-visible:!outline-none min-h-[50px] !pl-2 !pr-3 !py-8')}
      variant={props.isFocused ? 'shadow' : 'light'}
      color={props.isFocused ? 'secondary' : 'default'}
      radius="sm"
      onClick={props.setFocused}
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
