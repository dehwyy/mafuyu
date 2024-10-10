'use client'

import { useMemo } from 'react'
import clsx from 'clsx'
import { useAtomValue } from 'jotai'

import { TransformTranslate } from '@/lib/const'
import { NavbarAtom } from '@/lib/store/global'
import ChatInformation from './ChatInformation'
import ChatInput from './ChatInput'
import ChatList from './ChatList'

export default function ChatLayout({ children }: { children: React.ReactNode }) {
  const navbar = useAtomValue(NavbarAtom)
  const h = useMemo(() => {
    return 589 + 52 + (navbar.isExpanded ? 0 : TransformTranslate.NavbarHide) + 'px'
  }, [navbar.isExpanded])
  return (
    <div className="w-full h-full flex pl-5">
      <ChatList />
      <div className={clsx('w-full flex flex-col flex-1 p-3 justify-between', '')}>
        <ChatInformation />
        <div
          style={{ height: h, maxHeight: h }}
          className={clsx('overflow-y-auto -mt-[64px] pt-[76px] pr-3 grid place-items-center transition-all')}
        >
          <section className="flex-1 flex flex-col items-start gap-y-3 max-w-full w-[600px]">{children}</section>
        </div>
        <ChatInput />
      </div>
    </div>
  )
}
