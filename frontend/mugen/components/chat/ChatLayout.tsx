'use client'

import { Divider, Input } from '@nextui-org/react'
import clsx from 'clsx'

import ChatInformation from './ChatInformation'
import ChatInput from './ChatInput'
import ChatList from './ChatList'

export default function ChatLayout({ children }: { children: React.ReactNode }) {
  return (
    <div className="w-full h-full flex">
      <ChatList />
      <Divider orientation="vertical" />
      <div className={clsx('w-full flex flex-col gap-y-5 flex-1 p-3', '')}>
        <ChatInformation />
        <div className="h-[549px] max-h-[549px] overflow-y-auto pr-3">
          <section className="flex-1 flex flex-col items-start gap-y-3">{children}</section>
        </div>
        <ChatInput />
      </div>
    </div>
  )
}
