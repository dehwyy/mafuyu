import { useMemo } from 'react'
import { Avatar, Card, CardBody } from '@nextui-org/react'
import { ChatAppendix } from '$icons/ChatAppendix'
import clsx from 'clsx'

import { Dev } from '@/lib/const'

interface MessageProps {
  currentUserId: string

  senderUserId: string
  senderUsername: string
  senderUserImage?: string

  nextMessageSenderUserId?: string

  // todo
  messageType: any
  messageContent: any
  messageTimestamp: string
}

interface MessageListProps {
  items: MessageProps[]
}

export function MessageList({}: MessageListProps) {
  return (
    <div className="w-[600px] flex flex-col gap-y-2">
      {new Array(20).fill(0).map((_, i) => (
        <Message
          key={i}
          currentUserId="user_true"
          senderUserId={'user_' + String(i % 4 == 0)}
          senderUsername="dehwyy"
          senderUserImage={Dev.Img}
          nextMessageSenderUserId={'user_' + String((i + 1) % 4 == 0 || i == 19)}
          messageType="plain/text"
          messageContent={"It's a message!"}
          messageTimestamp="20:31"
        />
      ))}
    </div>
  )
}

function Message(props: MessageProps) {
  const isCurrentUser = useMemo(() => {
    return props.currentUserId === props.senderUserId
  }, [])

  const isMessageBatch = useMemo(() => {
    return props.nextMessageSenderUserId && props.nextMessageSenderUserId === props.senderUserId
  }, [props.senderUserId, props.nextMessageSenderUserId, isCurrentUser])

  return (
    <article className={clsx({ 'self-end': isCurrentUser }, 'max-w-[60%] min-w-[30%] flex items-end')}>
      {!isCurrentUser &&
        (isMessageBatch ? (
          <div className="w-[40px] h-[40px]" />
        ) : (
          <Avatar
            src={props.senderUserImage || Dev.Img}
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
            {!isCurrentUser && <p className="mb-1 text-[12px] font-bold text-secondary-500">{props.senderUsername}</p>}
            <div>
              <p>{props.messageContent}</p>
              <p className="absolute bottom-0.5 right-3 text-[10px] font-semibold  select-none text-foreground/40">
                {props.messageTimestamp}
              </p>
            </div>
          </CardBody>
        </Card>
      </div>
    </article>
  )
}
