import { BreadcrumbItem, Breadcrumbs, Button, ButtonGroup, Input, Tooltip } from '@nextui-org/react'

import { IconFile } from '@/components/icons/File'
import { IconMicrophone } from '@/components/icons/Microphone'
import { IconPencil } from '@/components/icons/Pencil'
import { IconPin } from '@/components/icons/Pin'
import { IconSearch } from '@/components/icons/Search'
import { IconTerminal } from '@/components/icons/Terminal'

interface ViewOverviewProps {}

const pathSegments = ['TextChat', 'Memories', '@Thread']

const buttons = [
  { icon: IconPin, text: 'Pinned messages' },
  { icon: IconPencil, text: 'Edit channel' },
  { icon: IconSearch, text: 'Search' },
  { icon: IconTerminal, text: 'View logs' }
]

export function ViewOverview(props: ViewOverviewProps) {
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
          {buttons.map((btn, i) => (
            <Tooltip content={btn.text} placement="bottom" showArrow closeDelay={0} delay={0}>
              <Button
                className="bg-transparent stroke-default-500 stroke-2 hover:stroke-default-800"
                disableAnimation
                disableRipple
                isIconOnly
              >
                <btn.icon className="stroke-inherit transition-all" />
              </Button>
            </Tooltip>
          ))}
        </div>
      </div>
    </section>
  )
}
