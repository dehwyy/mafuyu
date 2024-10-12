'use client'

import clsx from 'clsx'

import ChatInput from '../input'
import ChatInformation from './ChatInformation'
import ChatList from './ChatList'
import Message from './Message'

const m = new Array(20).fill({})

export default function ChatLayout() {
  return (
    <div className="w-full flex pl-5 transition-all h-full max-h-full">
      <ChatList />
      <div className={clsx('w-full flex flex-col flex-1 p-3 justify-between', '')}>
        <ChatInformation />
        <div className={clsx('overflow-y-auto -mt-[64px] pt-[72px] pb-3 pr-3 grid place-items-center transition-all')}>
          <section className="flex-1 flex flex-col items-start gap-y-3 max-w-full w-[600px]">
            {m.map((v, i) => (
              <Message
                key={i}
                senderUserId={'user_' + String(i % 4 == 0)}
                nextMessageSenderUserId={'user_' + String((i + 1) % 4 == 0 || m[i + 1] === undefined)}
                isCurrentUser={i % 4 == 0}
              />
            ))}
          </section>
        </div>
        <ChatInput />
      </div>
    </div>
  )
}
