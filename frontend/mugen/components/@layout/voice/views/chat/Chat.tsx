import { Avatar, Image } from '@nextui-org/react'

import { Dev } from '@/lib/const'
import { MessageImage, MessageText } from '@/lib/dto/voice/message'
import { ChatMessage } from './ChatMessage'

const t =
  'BBauman Moscow State Technical University (BMSTU) is one of the most prestigious technical universities in Russia.Bauman Moscow State Technical University (BMSTU) is one of the most prestigious technical universities in Russia.auman Moscow State Technical University (BMSTU) is one of the most prestigious technical universities in Russia.'
interface ViewChatProps {}

export function ViewChat(props: ViewChatProps) {
  return (
    <section className="h-full flex flex-col gap-y-5">
      <ChatMessage
        messageId={Dev.MessageId1}
        senderUsername="dehwyy"
        senderImage={Dev.Img}
        time="10/14/2024 8:31 PM"
        component={new MessageText(t)}
      />
      <ChatMessage
        messageId={Dev.MessageId2}
        senderUsername="dehwyy"
        senderImage={Dev.Img}
        time="10/14/2024 8:31 PM"
        component={new MessageImage(Dev.Img2, 'image')}
      />
      <ChatMessage
        messageId={Dev.MessageId3}
        senderUsername="dehwyy"
        senderImage={Dev.Img}
        time="10/14/2024 8:31 PM"
        component={new MessageImage('https://media1.tenor.com/m/uSA4mIkGS6wAAAAd/wind-breaker-sakura.gif', 'wind-breaker')}
      />
    </section>
  )
}
