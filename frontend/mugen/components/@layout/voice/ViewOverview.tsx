import { BreadcrumbItem, Breadcrumbs, Button, ButtonGroup, Input } from '@nextui-org/react'

import { IconFile } from '@/components/icons/File'
import { IconMicrophone } from '@/components/icons/Microphone'

interface ViewOverviewProps {}

export function ViewOverview(props: ViewOverviewProps) {
  return (
    <section className="w-full h-full flex items-center justify-between">
      <div>
        <Breadcrumbs>
          <BreadcrumbItem>Home</BreadcrumbItem>
          <BreadcrumbItem>Music</BreadcrumbItem>
          <BreadcrumbItem>Artist</BreadcrumbItem>
          <BreadcrumbItem>Album</BreadcrumbItem>
          <BreadcrumbItem>Song</BreadcrumbItem>
        </Breadcrumbs>
      </div>
      <div>
        <ButtonGroup>
          {new Array(3).fill(0).map((_, i) => (
            <Button
              key={i}
              variant="light"
              size="sm"
              startContent={<IconFile />}
            >
              Action {i}
            </Button>
          ))}
        </ButtonGroup>
      </div>
    </section>
  )
}
