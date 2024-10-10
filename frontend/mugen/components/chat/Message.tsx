'use client'

import { useMemo } from 'react'
import { Avatar, Card, CardBody, CardFooter, CardHeader } from '@nextui-org/react'
import clsx from 'clsx'

import { Dev } from '@/lib/const'
import { ChatAppendix } from '../icons/ChatAppendix'

interface MessageProps {
  isCurrentUser: boolean
  senderUserId: string
  nextMessageSenderUserId?: string
}

export default function Message({ isCurrentUser, senderUserId, nextMessageSenderUserId }: MessageProps) {
  const isMessageBatch = useMemo(() => {
    return nextMessageSenderUserId && nextMessageSenderUserId === senderUserId
  }, [senderUserId, nextMessageSenderUserId, isCurrentUser])

  return (
    <article className={clsx(isCurrentUser && 'self-end', 'max-w-[60%] min-w-[30%] flex items-end')}>
      {!isCurrentUser &&
        (isMessageBatch ? (
          <div className="w-[40px] h-[40px]" />
        ) : (
          <Avatar
            src={Dev.Img}
            className="mb-1"
          />
        ))}
      <div className="flex items-end">
        {!isCurrentUser && <ChatAppendix className="fill-default-300/20" />}
        <Card
          className={clsx('dark:bg-default-300/20 !overflow-visible', !isCurrentUser && 'rounded-bl-none !shadow-none')}
          isBlurred
          shadow="sm"
          radius="lg"
        >
          <CardBody className="!pt-2 !pb-2.5 !pl-4 !pr-14 text-sm overflow-visible">
            {!isCurrentUser && <p className="mb-1 text-[12px] font-bold">Waypo1nt</p>}
            <div>
              <p>a message to user?</p>
              <p className="absolute bottom-0.5 right-3 text-[10px] select-none text-foreground/40">20:31</p>
            </div>
          </CardBody>
        </Card>
      </div>
    </article>
  )
}
