import { Avatar } from '@nextui-org/react';



import { Dev } from '@/lib/const';
import { MessageProps } from '@/lib/dto/voice/message';
import { UserCardPopover } from '../../UserCardPopover';


const offsetTop = -30

interface ChatMessageProps extends MessageProps {
  disableUserCardPopover?: boolean
}

interface WrapperProps {
  children: React.ReactNode
  withPopover?: boolean

  username: string
  userImage?: string
}

export function ChatMessage(props: ChatMessageProps) {
  return (
    <div id={props.messageId} className="flex gap-x-4 px-3 py-4">
      <div className="mt-1">
        <AvatarPopoverWrapper withPopover={!props.disableUserCardPopover} username={props.senderUsername} userImage={props.senderImage}>
          <Avatar size="md" src={props.senderImage || Dev.Img} />
        </AvatarPopoverWrapper>
      </div>
      <div>
        <div className="font-medium mb-1 flex gap-x-3 items-center">
          <UserCardPopoverWrapper withPopover={!props.disableUserCardPopover} username={props.senderUsername} userImage={props.senderImage}>
            <span className="text-[16px] text-default-800 font-semibold">{props.senderUsername}</span>
          </UserCardPopoverWrapper>
          <span className="text-[12px] text-default-400 mt-0.5">{props.time}</span>
        </div>
        {props.component.build()}
      </div>
    </div>
  )
}

function AvatarPopoverWrapper(props: WrapperProps) {
  return props.withPopover ? (
    <UserCardPopover
      popoverProps={{ placement: 'right-start', crossOffset: offsetTop, offset: 16 }}
      username={props.username}
      userImage={props.userImage}
    >
      <button>{props.children}</button>
    </UserCardPopover>
  ) : (
    props.children
  )
}

function UserCardPopoverWrapper(props: WrapperProps) {
  return props.withPopover ? (
    <UserCardPopover
      popoverProps={{ placement: 'right-start', crossOffset: offsetTop }}
      username={props.username}
      userImage={props.userImage}
      classNameOpen="text-default-900"
      className="text-default-600 hover:text-default-900"
    >
      <button>{props.children}</button>
    </UserCardPopover>
  ) : (
    props.children
  )
}
