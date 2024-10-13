import { Avatar, Image } from '@nextui-org/react'

import { Dev } from '@/lib/const'

const t =
  'BBauman Moscow State Technical University (BMSTU) is one of the most prestigious technical universities in Russia.Bauman Moscow State Technical University (BMSTU) is one of the most prestigious technical universities in Russia.auman Moscow State Technical University (BMSTU) is one of the most prestigious technical universities in Russia.'
interface ViewChatProps {}

export function ViewChat(props: ViewChatProps) {
  return (
    <section className="h-full flex flex-col gap-y-5">
      <MessageContainer>
        <p className="text-[15px] font-normal text-default-700">{t}</p>
      </MessageContainer>
      <MessageContainer>
        <Image
          src={Dev.Img2}
          className="w-[200px] h-[200px]"
        />
      </MessageContainer>
      <MessageContainer>
        <Image
          src="https://media1.tenor.com/m/uSA4mIkGS6wAAAAd/wind-breaker-sakura.gif"
          className="h-[200px]"
        />
      </MessageContainer>
    </section>
  )
}

function MessageContainer({ children }: { children: React.ReactNode }) {
  return (
    <div className="flex gap-x-4 px-3 py-4">
      <div className="mt-1">
        <Avatar src={Dev.Img} />
      </div>
      <div>
        <p className="font-medium">
          <span className="text-[16px] mr-3">dehwyy</span>
          <span className="text-[12px] text-default-400">09/26/2024 9:29 PM</span>
        </p>
        {children}
      </div>
    </div>
  )
}
