import { BreadcrumbItem, Breadcrumbs, Button, ButtonGroup, Input, Tooltip } from '@nextui-org/react'

import { IconFile } from '@/components/icons/File'
import { IconMicrophone } from '@/components/icons/Microphone'
import { IconPencil } from '@/components/icons/Pencil'
import { IconPin } from '@/components/icons/Pin'
import { IconSearch } from '@/components/icons/Search'
import { IconTerminal } from '@/components/icons/Terminal'
import { ChatOverview } from './views/chat/ChatOverview'

interface ViewOverviewProps {}

const pathSegments = ['TextChat', 'Memories', '@Thread']

const buttons = [
  { icon: IconPin, text: 'Pinned messages' },
  { icon: IconPencil, text: 'Edit channel segment' },
  { icon: IconSearch, text: 'Search' },
  { icon: IconTerminal, text: 'View audit logs' }
]

export function ViewOverview(props: ViewOverviewProps) {
  return <ChatOverview />
}
