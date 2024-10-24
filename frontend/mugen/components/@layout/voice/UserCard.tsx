import { Avatar, Button, Card, CardBody, CardFooter, CardHeader, Chip, Image } from '@nextui-org/react'
import clsx from 'clsx'

import { Dev } from '@/lib/const'

interface UserCardProps {
  username: string
  userImage?: string
}

const userBg = '#9d00ff'

const userRoles = [
  { name: 'III', color: 'secondary' },
  { name: 'II', color: 'primary' },
  { name: 'I', color: 'danger' },
  { name: 'III', color: 'secondary' },
  { name: 'Cofe', color: 'warning' },
  { name: 'Rot', color: 'danger' },
  { name: 'Brainless', color: 'secondary' },
  { name: 'Common user', color: 'primary' },
  { name: 'Admin', color: 'success' }
] as const
export function UserCard(props: UserCardProps) {
  return (
    <Card className="!p-0 w-[300px]">
      <CardHeader className="p-0">
        <div style={{ background: userBg }} className={clsx('w-full h-[105px]')} />
      </CardHeader>
      <CardBody className="relative overflow-y-visible py-2">
        <div className="h-[80px] w-[80px] absolute -top-[40px] z-50">
          <Image radius="full" src={props.userImage || Dev.Img} alt="avatar" className="object-cover" />
        </div>
        <div className="mt-[40px] pt-2 pb-4 flex flex-col gap-y-1">
          <p className="font-bold text-[20px] leading-[24px]">{props.username}</p>
          <p className="text-[14px] leading-[18px]">{props.username}</p>
        </div>
        <div className="flex gap-x-1 gap-y-1 flex-wrap">
          {userRoles.map((role, i) => (
            <Chip key={i} color={role.color} variant="dot">
              {role.name}
            </Chip>
          ))}
        </div>
      </CardBody>
      <CardFooter className="flex justify-between gap-x-5">
        <Button color="secondary" variant="bordered" className="w-full">
          Send message
        </Button>
      </CardFooter>
    </Card>
  )
}
