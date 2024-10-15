import { useHover } from '@custom-react-hooks/all'
import { Button } from '@nextui-org/button'
import clsx from 'clsx'

import { IconMessages } from '@/components/icons/Messages'
import { IconSettings } from '@/components/icons/Settings'
import { IconVolume } from '@/components/icons/Volume'
import { ChatMember, ChatMemberProps } from './ChatMember'

interface ChatContainerProps {
  name: string
  type: 'voice' | 'text'
  items?: ChatMemberProps[]
}

export function ChatContainer(props: ChatContainerProps) {
  const { isHovered, ref } = useHover<HTMLDivElement>()

  return (
    <>
      <div ref={ref} className="h-[34px] cursor-pointer hover:bg-default-100 transition-all px-3 rounded-md">
        <div className="flex gap-x-1 items-center h-full">
          {props.type === 'text' ? (
            <IconMessages className="h-5 min-w-5" />
          ) : props.type === 'voice' ? (
            <IconVolume className="h-5 min-w-5" />
          ) : null}
          <p className="bold text-[15px] font-medium ellipsis">{props.name}</p>

          <Button
            isIconOnly
            className={clsx(isHovered ? 'opacity-100' : 'opacity-0', 'bg-transparent ml-auto !p-1 !min-w-0 !w-auto !h-auto')}
          >
            <IconSettings className="h-5 min-w-5" />
          </Button>
        </div>
      </div>
      {props.type === 'voice' && (
        <div className="ml-6 mr-1 cursor-pointer">
          <div className="items-start mt-0.5 flex flex-col gap-y-1">{props.items?.map((item, i) => <ChatMember key={i} {...item} />)}</div>
        </div>
      )}
    </>
  )
}
