import { Button, User } from '@nextui-org/react'

import { IconSettings } from '@/components/icons/Settings'

interface UserCardProps {
  username: string
  userImage?: string
  userStatus?: string
}

export function UserPanel(props: UserCardProps) {
  return (
    <div className="flex justify-between w-full">
      <User
        classNames={{
          base: 'font-semibold',
          wrapper: '!ml-1',
          description: 'text-success-400/60'
        }}
        name={props.username}
        avatarProps={{ src: props.userImage }}
        description={props.userStatus}
      />
      <Button isIconOnly className="bg-transparent focus-visible:!outline-none outline-none">
        <IconSettings />
      </Button>
    </div>
  )
}
