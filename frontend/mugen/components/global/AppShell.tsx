import clsx from 'clsx'

import Navbar from '@/components/global/Navbar'
import Sidebar from '@/components/global/Sidebar'
import { ComponentSize } from '@/lib/const'

export default function AppShell({ children }: { children: React.ReactNode }) {
  return (
    <div className="flex justify-center min-h-screen">
      <div className="flex flex-col w-[900px] max-w-full relative">
        <Navbar />
        <main className="flex-1">{children}</main>
      </div>
    </div>
  )
}
