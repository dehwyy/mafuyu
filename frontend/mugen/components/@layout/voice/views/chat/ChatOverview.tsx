import { BreadcrumbItem, Breadcrumbs, Button, ButtonGroup, Input, Tooltip } from '@nextui-org/react'

import { IconFile } from '@/components/icons/File'
import { IconMicrophone } from '@/components/icons/Microphone'
import { IconPencil } from '@/components/icons/Pencil'
import { IconPin } from '@/components/icons/Pin'
import { IconSearch } from '@/components/icons/Search'
import { IconTerminal } from '@/components/icons/Terminal'
import { Dev } from '@/lib/const'
import { MessageImage, MessageText } from '@/lib/dto/voice/message'
import { ChatOverviewActionTrigger } from './ChatOverviewActionTrigger'
import { ChatOverviewActionViewAuditLogs } from './ChatOverviewActionViewAuditLogs'
import { ChatOverviewActionViewPinnedMessages } from './ChatOverviewActionViewPinnedMessages'
import { ChatOverviewActionViewSearch } from './ChatOverviewActionViewSearch'
import { ChatOverviewActionViewSegmentEdit } from './ChatOverviewActionViewSegmentEdit'

interface ViewOverviewProps {}

const pathSegments = ['TextChat', 'Memories', '@Thread']
const pinnedMessage = [
  {
    messageId: Dev.MessageId1,
    senderUsername: 'Mugen',
    senderImage: Dev.Img2,
    time: '10/14/2024 8:31 PM',
    component: new MessageImage('https://media1.tenor.com/m/uSA4mIkGS6wAAAAd/wind-breaker-sakura.gif', 'wind-breaker', {
      h: 160
    })
  },
  {
    messageId: Dev.MessageId2,
    senderUsername: 'dehwyy',
    senderImage: Dev.Img,
    time: '10/14/2024 8:31 PM',
    component: new MessageText("It's a message!")
  }
]

const buttons = [
  { triggerIcon: IconSearch, viewComponent: ChatOverviewActionViewSearch, text: 'Search' },
  { triggerIcon: IconTerminal, viewComponent: ChatOverviewActionViewAuditLogs, text: 'View audit logs' }
]

export function ChatOverview(props: ViewOverviewProps) {
  return (
    <section className="w-full h-full flex items-center justify-between">
      <div className="select-none">
        <Breadcrumbs>
          {pathSegments.map((segment, i) => (
            <BreadcrumbItem key={i}>{segment}</BreadcrumbItem>
          ))}
        </Breadcrumbs>
      </div>
      <div>
        <div className="flex gap-x-4">
          <ChatOverviewActionTrigger triggerIcon={IconPin} text="Show pinned messages">
            <ChatOverviewActionViewPinnedMessages pinnedMessage={pinnedMessage} />
          </ChatOverviewActionTrigger>
          <ChatOverviewActionTrigger triggerIcon={IconPencil} text="Edit channel segment">
            <ChatOverviewActionViewSegmentEdit channelName="TextChannelName" />
          </ChatOverviewActionTrigger>

          {buttons.map((item, i) => (
            <ChatOverviewActionTrigger key={i} triggerIcon={item.triggerIcon} text={item.text}>
              <item.viewComponent />
            </ChatOverviewActionTrigger>
          ))}
        </div>
      </div>
    </section>
  )
}
