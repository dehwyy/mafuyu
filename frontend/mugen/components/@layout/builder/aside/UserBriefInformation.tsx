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
    <User
      name={name}
      description={description}
      avatarProps={{ src: avatarSrc }}
      className={clsx(className)}
    />
  )
}
