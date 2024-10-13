import { useEffectOnce } from '@custom-react-hooks/all'
import { Card, CardBody, ScrollShadow } from '@nextui-org/react'
import clsx from 'clsx'

interface BoxProps {
  children: React.ReactNode
  className?: string

  // Pass to `Wrapper`.
  grow?: boolean
  h?: string
  scrollable?: boolean
}

export function Box({ children, className, ...props }: BoxProps) {
  return (
    <Wrapper {...props}>
      <Card
        isBlurred
        shadow="sm"
        style={{ height: props.h, minHeight: props.h, maxHeight: props.h }}
        className="border-none bg-background/60 dark:bg-gradient-to-br from-default-100/30 to-default-100/10 min-h-full"
      >
        <CardBody className={clsx('flex flex-col gap-y-2 overflow-y-auto', className)}>{children}</CardBody>
      </Card>
    </Wrapper>
  )
}

interface WrapperProps {
  children: React.ReactNode
  scrollable?: boolean
  grow?: boolean
}

function Wrapper(props: WrapperProps) {
  if (props.scrollable) {
    return (
      <ScrollShadow
        style={{ height: '100%' }}
        className={clsx('pr-1', props.grow && 'flex-1')}
      >
        {props.children}
      </ScrollShadow>
    )
  } else {
    return <>{props.children}</>
  }
}
