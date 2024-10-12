import { Card, CardBody, User } from '@nextui-org/react'

import { Dev } from '@/lib/const'

export const UserShort = () => {
  return (
    <Card
      isBlurred
      shadow="sm"
      className="min-h-[64px] border-none bg-background/60 dark:bg-gradient-to-br from-default-100/30 to-default-100/10"
    >
      <CardBody>
        <User
          name="dehwyy"
          description="awesome user"
          avatarProps={{ src: Dev.Img }}
          className="self-start"
        />
      </CardBody>
    </Card>
  )
}
