'use client'

import { Card, CardBody, CardFooter, CardHeader } from '@nextui-org/react'
import clsx from 'clsx'

interface MessageProps {
  alignRight?: boolean
}

export default function Message({ alignRight }: MessageProps) {
  console.log(alignRight)
  return (
    <Card
      className={clsx(alignRight && 'self-end', 'dark:bg-primary-300/20 max-w-[60%] min-w-[30%]')}
      isBlurred
      radius="sm"
      shadow="sm"
    >
      <CardBody className="!py-2 !px-5 text-sm">
        <p className="mb-3">Waypo1nt</p>
        <p>a message to user?</p>
      </CardBody>
    </Card>
  )
}
