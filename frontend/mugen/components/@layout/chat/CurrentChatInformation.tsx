import { Avatar } from '@nextui-org/react'

interface CurrentChatInformationProps {
  avatarSrc?: string
  name: string
}

export function CurrentChatInformation({ avatarSrc, name }: CurrentChatInformationProps) {
  return (
    <div className="flex gap-x-3 items-center">
      <Avatar src={avatarSrc} />
      <p className="font-semibold">{name}</p>
    </div>
  )
}
