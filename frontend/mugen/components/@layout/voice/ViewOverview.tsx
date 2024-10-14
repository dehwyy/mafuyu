import { BreadcrumbItem, Breadcrumbs, Button, ButtonGroup, Input } from '@nextui-org/react'

import { IconFile } from '@/components/icons/File'
import { IconMicrophone } from '@/components/icons/Microphone'
import { IconPencil } from '@/components/icons/Pencil'
import { IconPin } from '@/components/icons/Pin'
import { IconSearch } from '@/components/icons/Search'
import { IconTerminal } from '@/components/icons/Terminal'

interface ViewOverviewProps {}

const pathSegments = ['TextChat', 'Memories', '@Thread']
const buttons = [<IconPin />, <IconPencil />, <IconSearch />, <IconTerminal />]

export function ViewOverview(props: ViewOverviewProps) {
  return (
    <section className="w-full h-full flex items-center justify-between">
      <div>
        <Breadcrumbs>
          {pathSegments.map((segment, i) => (
            <BreadcrumbItem key={i}>{segment}</BreadcrumbItem>
          ))}
        </Breadcrumbs>
      </div>
      <div>
        <div className="flex gap-x-2">
          {buttons.map((button, i) => (
            <Button
              key={i}
              size="sm"
              className="bg-transparent"
              isIconOnly
            >
              {button}
            </Button>
          ))}
        </div>
      </div>
    </section>
  )
}
