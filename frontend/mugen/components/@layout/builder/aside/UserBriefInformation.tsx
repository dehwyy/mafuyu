import { Card, CardBody, User } from '@nextui-org/react'
import clsx from 'clsx'

interface UserBriefInformationProps {
  name: string
  description: string
  avatarSrc: string
  className?: string
}

export function UserBriefInformation({ name, description, avatarSrc, className }: UserBriefInformationProps) {
  return (
    <Card
      isBlurred
      shadow="sm"
      className="h-full border-none bg-background/60 dark:bg-gradient-to-br from-default-100/30 to-default-100/10 max-h-full"
    >
      <CardBody className="!p-0 h-full grid place-items-center">
        <User
          name={name}
          description={description}
          avatarProps={{ src: avatarSrc }}
          className={clsx(className)}
        />
      </CardBody>
    </Card>
  )
}
