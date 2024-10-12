import clsx from 'clsx'

interface ScrollableProps {
  children: React.ReactNode[] | React.ReactNode
}

export function Scrollable({ children }: ScrollableProps) {
  return (
    <div className={clsx('overflow-y-auto -mt-[64px] pt-[72px] pb-3 pr-3 grid place-items-center transition-all')}>
      <section className="flex-1 flex flex-col items-start gap-y-3 max-w-full w-[600px]">{children}</section>
    </div>
  )
}
