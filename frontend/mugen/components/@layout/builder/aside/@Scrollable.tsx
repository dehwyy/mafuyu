import { Card, CardBody, ScrollShadow } from '@nextui-org/react'

interface ScrollableProps {
  children: React.ReactNode[] | React.ReactNode
}

export function Scrollable({ children }: ScrollableProps) {
  return (
    <ScrollShadow
      style={{ maxHeight: 'calc(100% - 70px)' }}
      className="mb-3 pr-1 transition-all"
    >
      <Card className="bg-default-100/50">
        <CardBody className="flex flex-col gap-y-2 !px-1 overflow-y-auto">{children}</CardBody>
      </Card>
    </ScrollShadow>
  )
}
