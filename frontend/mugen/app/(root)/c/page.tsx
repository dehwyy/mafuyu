import ChatLayout from '@/components/chat/ChatLayout'
import Message from '@/components/chat/Message'

export default function $Page() {
  const m = new Array(10).fill({})

  return (
    <ChatLayout>
      {m.map((_, i) => (
        <Message
          key={i}
          alignRight={i % 4 == 0}
        />
      ))}
    </ChatLayout>
  )
}
