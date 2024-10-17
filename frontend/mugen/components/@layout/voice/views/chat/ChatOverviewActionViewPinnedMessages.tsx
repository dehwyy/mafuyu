import { Card, CardBody, CardFooter, CardHeader, Divider } from '@nextui-org/react'

import { IconPin } from '@/components/icons/Pin'
import { CardPreseted } from '@/components/reusable/CardPreseted'
import { MessageProps } from '@/lib/dto/voice/message'
import { ChatMessage } from './ChatMessage'

interface ChatOverviewActionViewPinnedMessagesProps {
  pinnedMessage: MessageProps[]
}

export function ChatOverviewActionViewPinnedMessages(props: ChatOverviewActionViewPinnedMessagesProps) {
  return (
    <CardPreseted variant="gradient">
      <CardHeader>
        <div className="flex gap-x-2 items-center">
          <IconPin className="stroke-2" />
          <p className="text-lg">Pinned messages</p>
        </div>
      </CardHeader>
      <Divider />
      <CardBody className="w-[350px]">
        {props.pinnedMessage.map((item, i) => (
          <ChatMessage key={i} {...item} />
        ))}
      </CardBody>
      {props.pinnedMessage.length === 0 && (
        <>
          <Divider />
          <CardFooter>
            <div className="w-full py-2">
              <p className="text-secondary-500 text-center font-semibold">No pinned messages yet!</p>
            </div>
          </CardFooter>
        </>
      )}
    </CardPreseted>
  )
}
