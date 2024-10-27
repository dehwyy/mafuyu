import clsx from 'clsx'

import { IconMessages } from '@/components/icons/Messages'
import { IconSettings } from '@/components/icons/Settings'
import { IconVolume } from '@/components/icons/Volume'
import { Dev } from '@/lib/const'
import { ChatContainer } from './laside/ChatContainer'
import { ServerOptions } from './laside/ServerOptions'

interface CurrentServerProps {
  className?: string
}

export function CurrentServer({ className }: CurrentServerProps) {
  return (
    <section className={clsx('flex-1 h-full flex flex-col gap-y-4 relative font-sans select-none', className)}>
      <ServerOptions serverName="Divine Crown" userRole={Dev.UserRole} />

      <div className="flex flex-col gap-y-0.5 px-1">
        <ChatContainer name="Osu!" type="text" />
        <ChatContainer name="Valorant" type="text" />
        <ChatContainer
          name="Voice"
          type="voice"
          items={[
            { username: 'dehwyy', userImage: Dev.Img },
            { username: 'Ash', userImage: Dev.Img2 }
          ]}
        />
      </div>
    </section>
  )
}
