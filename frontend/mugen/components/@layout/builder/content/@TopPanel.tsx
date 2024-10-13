import { Card, CardBody } from '@nextui-org/react'

interface TopPanelProps {
  children: React.ReactNode[] | React.ReactNode
  size?: 'sm' | 'md'
}

const smHeight = 48
const mdHeight = 64

export function TopPanel({ children, size }: TopPanelProps) {
  return (
    <div className="pr-5">
      <Card
        isBlurred
        shadow="lg"
        className="border-none dark:bg-gradient-to-br from-default-100/30 to-default-100/10 relative z-10"
        style={{ height: size === 'sm' ? smHeight : mdHeight }}
      >
        <CardBody className="px-5 flex flex-row items-center gap-x-5 text-sm overflow-hidden">{children}</CardBody>
      </Card>
    </div>
  )
}
