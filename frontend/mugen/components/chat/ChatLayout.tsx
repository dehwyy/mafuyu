'use client'

import { useMemo } from 'react'
import clsx from 'clsx'
import { useAtomValue } from 'jotai'

import { TransformTranslate } from '@/lib/const'
import { NavbarAtom } from '@/lib/store/global'
import ChatInput from '../input'
import ChatInformation from './ChatInformation'
import ChatList from './ChatList'

export default function ChatLayout({ children }: { children: React.ReactNode }) {
  const navbar = useAtomValue(NavbarAtom)

  const deltaH = useMemo(() => {
    return 20 + (navbar.isExpanded ? TransformTranslate.NavbarHide : 0) + 'px'
  }, [navbar.isExpanded])

  return (
    <div
      style={{ height: `calc(100vh - ${deltaH})`, maxHeight: `calc(100vh - ${deltaH})` }}
      className="w-full flex pl-5 transition-all"
    >
      <ChatList />
      <div className={clsx('w-full flex flex-col flex-1 p-3 justify-between', '')}>
        <ChatInformation />
        <div className={clsx('overflow-y-auto -mt-[64px] pt-[72px] pb-3 pr-3 grid place-items-center transition-all')}>
          <section className="flex-1 flex flex-col items-start gap-y-3 max-w-full w-[600px]">{children}</section>
        </div>
        <ChatInput />
      </div>
    </div>
  )
}
