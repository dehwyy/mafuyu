import { Avatar } from '@nextui-org/react'

import { Dev } from '@/lib/const'
import { MessageProps } from '@/lib/dto/voice/message'
import { UserCardPopover } from '../../UserCardPopover'

const offsetTop = -30

export function ChatMessage(props: MessageProps) {
  return (
    <div id={props.messageId} className="flex gap-x-4 px-3 py-4">
      <div className="mt-1">
        <UserCardPopover
          popoverProps={{ placement: 'right-start', crossOffset: offsetTop, offset: 16 }}
          username={props.senderUsername}
          userImage={props.senderImage}
        >
          <button>
            <Avatar size="md" src={props.senderImage || Dev.Img} />
          </button>
        </UserCardPopover>
      </div>
      <div>
        <div className="font-medium mb-1 flex items-center">
          <UserCardPopover
            popoverProps={{ placement: 'right-start', crossOffset: offsetTop }}
            username={props.senderUsername}
            userImage={props.senderImage}
            classNameOpen="text-default-900"
            className="text-default-600 hover:text-default-900"
          >
            <button className="text-[16px] mr-3 ">{props.senderUsername}</button>
          </UserCardPopover>
          <span className="text-[12px] text-default-400">{props.time}</span>
        </div>
        {props.component.build()}
      </div>
    </div>
  )
}
