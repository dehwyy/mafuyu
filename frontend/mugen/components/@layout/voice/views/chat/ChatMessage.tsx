import { Avatar } from '@nextui-org/react'

import { Dev } from '@/lib/const'
import { MessageProps } from '@/lib/dto/voice/message'

export function ChatMessage(props: MessageProps) {
  return (
    <div id={props.messageId} className="flex gap-x-4 px-3 py-4">
      <div className="mt-1">
        <Avatar size="md" src={props.senderImage || Dev.Img} />
      </div>
      <div>
        <p className="font-medium mb-1">
          <span className="text-[16px] mr-3">{props.senderUsername}</span>
          <span className="text-[12px] text-default-400">{props.time}</span>
        </p>
        {props.component.build()}
      </div>
    </div>
  )
}
