import { Avatar, Card, CardBody } from '@nextui-org/react'
import clsx from 'clsx'

import { Dev } from '@/lib/const'

export default function ChatInformation() {
  return (
    <Card
      isBlurred
      shadow="sm"
      className="border-none dark:bg-default-300/40"
    >
      <CardBody className="flex flex-row items-center gap-x-3">
        <Avatar src={Dev.Img} />
        <p>dehwwyy</p>
      </CardBody>
    </Card>
  )
}
