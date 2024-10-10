import { Button, Card, CardBody } from '@nextui-org/react'
import clsx from 'clsx'

export default function ChatList() {
  return (
    <Card className={clsx('w-[200px] my-3')}>
      <CardBody className="flex flex-col gap-y-2 !px-1 h-[693px] overflow-y-auto">
        {new Array(20).fill(0).map((_, i) => (
          <ChatListItem key={i} />
        ))}
      </CardBody>
    </Card>
  )
}

function ChatListItem() {
  return (
    <Button
      className="w-full focus-visible:!outline-none min-h-[50px]"
      variant="light"
      radius="sm"
    >
      Chat's name
    </Button>
  )
}
