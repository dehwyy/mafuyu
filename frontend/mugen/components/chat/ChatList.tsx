import { Button, Card, CardBody } from '@nextui-org/react'
import clsx from 'clsx'

export default function ChatList() {
  return (
    <div className={clsx('w-[200px] bg-content-1')}>
      <aside className="flex flex-col gap-3 py-1 px-1">
        <ChatListItem />
      </aside>
    </div>
  )
}

function ChatListItem() {
  return (
    <Button
      className="w-full focus-visible:!outline-none"
      variant="light"
      radius="sm"
    >
      Chat's name
    </Button>
  )
}
