import { Card, CardBody, ScrollShadow } from '@nextui-org/react'
import clsx from 'clsx'

interface ScrollableProps {
  children: React.ReactNode[] | React.ReactNode
  className?: string
}

export function Scrollable({ children, className }: ScrollableProps) {
  return (
    <ScrollShadow
      style={{ height: 'calc(100% - 70px)', maxHeight: 'calc(100% - 70px)', minHeight: 'calc(100% - 70px)' }}
      className="mb-3 pr-1 transition-all"
    >
      <Card className="bg-default-100/50 min-h-full">
        <CardBody className={clsx('flex flex-col gap-y-2 !px-1 overflow-y-auto', className)}>{children}</CardBody>
      </Card>
    </ScrollShadow>
  )
}
