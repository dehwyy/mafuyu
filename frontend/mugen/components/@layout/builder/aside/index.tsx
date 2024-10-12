import { Scrollable } from './@Scrollable'
import { TopPanel } from './@TopPanel'
import { UserBriefInformation } from './UserBriefInformation'

interface AsideProps {
  children: React.ReactNode[] | React.ReactNode
}

export function Aside({ children }: AsideProps) {
  return <section className="flex flex-col gap-y-3 py-3 w-[250px]">{children}</section>
}

export { UserBriefInformation, TopPanel as AsideTopPanel, Scrollable as AsideScrollable }
