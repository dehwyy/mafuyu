import { useState } from 'react'
import { useHover } from '@custom-react-hooks/all'
import { Button, Card, CardBody, CardFooter, CardHeader, Divider } from '@nextui-org/react'
import clsx from 'clsx'

import { IconPencil } from '@/components/icons/Pencil'
import { IconPin } from '@/components/icons/Pin'
import { IconTrash } from '@/components/icons/Trash'
import { IconX } from '@/components/icons/X'
import { CardPreseted } from '@/components/reusable/CardPreseted'
import { LinkSmooth } from '@/components/reusable/LinkSmooth'
import { ModalRemove } from '@/components/reusable/ModalRemove'
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
      <CardBody className="w-[450px] max-h-[400px] overflow-y-auto">
        {props.pinnedMessage.map((item, i) => (
          <PinnedMessage key={i} {...item} />
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

function PinnedMessage(props: MessageProps) {
  const { isHovered, ref } = useHover<HTMLDivElement>()
  const [isRemoveModalOpen, setRemoveModalOpen] = useState(false)

  return (
    <>
      <LinkSmooth anchorId={props.messageId}>
        <article ref={ref} className="flex justify-between hover:bg-default-100 rounded-md transition-all duration-50 w-full">
          <ChatMessage {...props} />
          <div className={clsx(isHovered ? 'opacity-100 visible' : 'opacity-0 invisible', 'pt-5 transition-all')}>
            <Button
              onClick={(e) => {
                e.preventDefault()
                setRemoveModalOpen(true)
              }}
              isIconOnly
              className="bg-transparent hover:stroke-danger stroke-default-700"
              disableAnimation
              disableRipple
            >
              <IconX className="h-4 w-4 stroke-inherit transition-all" />
            </Button>
          </div>
        </article>
      </LinkSmooth>
      <ModalRemove
        isOpen={isRemoveModalOpen}
        setOpen={setRemoveModalOpen}
        title="Remove pinned message"
        description="Are you sure to remove it?"
      />
    </>
  )
}
