import clsx from 'clsx'

interface ScrollableProps {
  children: React.ReactNode[] | React.ReactNode
  contentWidth?: string
}

export function Scrollable({ children, contentWidth }: ScrollableProps) {
  return (
    <div className={clsx('overflow-y-auto -mt-[64px] pt-[72px] pb-3 pr-3 grid place-items-center transition-all')}>
      <section
        style={{ width: contentWidth ?? '100%' }}
        className="flex-1 flex flex-col items-start gap-y-3 max-w-full"
      >
        {children}
      </section>
    </div>
  )
}
