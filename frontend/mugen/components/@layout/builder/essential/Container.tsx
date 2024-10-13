import clsx from 'clsx'

interface AsideProps {
  width?: string
  horizontal?: boolean
  grow?: boolean
  children?: React.ReactNode[] | React.ReactNode
  className?: string
}

export function Container(props: AsideProps) {
  return (
    <section
      style={{
        width: props.width ?? '100%'
      }}
      className={clsx(
        'flex gap-y-3 gap-x-1 overflow-y-auto',
        props.className,
        props.grow && 'flex-1',
        props.horizontal ? 'flex-row' : 'flex-col'
      )}
    >
      {props.children}
    </section>
  )
}
