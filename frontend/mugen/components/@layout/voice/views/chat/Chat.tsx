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
<ChatMessage
        messageId={Dev.MessageId4}
        senderUsername="dehwyy"
        senderImage={Dev.Img}
        time="10/14/2024 8:31 PM"
        component={new MessageImage('https://media1.tenor.com/m/zw3QqwNB3yQAAAAC/%E9%84%B0%E5%BA%A7%E8%89%BE%E8%8E%89%E5%90%8C%E5%AD%B8-alya-sometimes-hides-her-feelings-in-russian.gif', 'wind-breaker')}
	/>
 <ChatMessage
        messageId={Dev.MessageId5}
        senderUsername="dehwyy"
        senderImage={Dev.Img}
        time="10/14/2024 8:31 PM"
        component={new MessageImage('https://media1.tenor.com/m/tucKAMTRacsAAAAC/akane-oshi-no-ko.gif', 'wind-breaker')}
      />
    </section>
  )
}
