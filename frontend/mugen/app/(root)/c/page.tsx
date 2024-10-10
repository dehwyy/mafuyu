import ChatLayout from '@/components/chat/ChatLayout'
import Message from '@/components/chat/Message'

export default function $Page() {
  const m = new Array(10).fill({})

  return (
    <ChatLayout>
      {m.map((v, i) => (
        <Message
          key={i}
          senderUserId={'user_' + String(i % 4 == 0)}
          nextMessageSenderUserId={'user_' + String((i + 1) % 4 == 0 || m[i + 1] === undefined)}
          isCurrentUser={i % 4 == 0}
        />
      ))}
    </ChatLayout>
  )
}
