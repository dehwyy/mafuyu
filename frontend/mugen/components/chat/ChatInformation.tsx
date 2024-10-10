import { Avatar, Card, CardBody } from '@nextui-org/react'
import clsx from 'clsx'

import { Dev } from '@/lib/const'

export default function ChatInformation() {
  return (
    <div className="pr-5">
      <Card
        isBlurred
        shadow="lg"
        className="border-none dark:bg-default-100/50 relative z-10"
      >
        <CardBody className="px-5 flex flex-row items-center gap-x-5">
          <Avatar src={Dev.Img} />
          <p className="font-semibold">dehwwyy</p>
        </CardBody>
      </Card>
    </div>
  )
}
