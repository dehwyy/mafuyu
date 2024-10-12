import { Card, CardBody } from '@nextui-org/react'

interface TopPanelProps {
  children: React.ReactNode[] | React.ReactNode
}

export function TopPanel({ children }: TopPanelProps) {
  return (
    <div className="pr-5">
      <Card
        isBlurred
        shadow="lg"
        className="border-none dark:bg-default-100/50 relative z-10"
      >
        <CardBody className="px-5 flex flex-row items-center gap-x-5">{children}</CardBody>
      </Card>
    </div>
  )
}
