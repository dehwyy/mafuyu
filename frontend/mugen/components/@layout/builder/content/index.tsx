import { BottomPanel } from './@BottomPanel'
import { Scrollable } from './@Scrollable'
import { TopPanel } from './@TopPanel'

interface ContentProps {
  children: React.ReactNode[] | React.ReactNode
}

export function Content({ children }: ContentProps) {
  return <div className="w-full flex flex-col flex-1 p-3 justify-between">{children}</div>
}

export { TopPanel as ContentTopPanel, Scrollable as ContentScrollable, BottomPanel as ContentBottomPanel }
