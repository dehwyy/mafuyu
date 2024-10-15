import { useMemo } from 'react';
import { Card, CardBody, ScrollShadow } from '@nextui-org/react';
import clsx from 'clsx';





const variants = {
  gradient: { c: 'bg-background/60 dark:bg-gradient-to-br from-default-100/30 to-default-100/10', shadow: 'sm', isBlured: true },
  gradientPurple: { c: 'bg-purple-600/60 dark:bg-gradient-to-br from-default-100/30 to-purple-400/30', shadow: 'sm', isBlured: true },
  unstyled: { c: 'bg-transparent shadow-none' }
} as {
  [key: string]: { c: string; shadow?: 'sm'; isBlured?: boolean }
}

interface WrapperProps {
  children: React.ReactNode
  h?: string
  w?: string
  scrollable?: boolean
  grow?: boolean
  alignSelf?: string
}

interface BoxProps extends WrapperProps {
  className?: string
  variant?: 'gradient' | 'gradientPurple' | 'unstyled'
}

export function Box({ children, className, ...props }: BoxProps) {
  const v = useMemo(() => {
    return variants[props.variant || 'unstyled']
  }, [props.variant])

  return (
    <Wrapper {...props}>
      <Card isBlurred={v.isBlured} shadow={v.shadow} className={clsx('min-h-full', v.c)}>
        <CardBody className={clsx('flex flex-col gap-y-2 overflow-y-auto', className)}>{children}</CardBody>
      </Card>
    </Wrapper>
  )
}

function Wrapper(props: WrapperProps) {
  if (props.scrollable) {
    return (
      <ScrollShadow style={{ height: props.h ?? '100%' }} className={clsx('pr-1', props.grow && 'flex-1')}>
        {props.children}
      </ScrollShadow>
    )
  } else {
    return (
      <div
        style={{ height: props.h, minHeight: props.h, maxHeight: props.h, width: props.w, alignSelf: props.alignSelf }}
        className={clsx(props.grow && 'flex-1')}
      >
        {props.children}
      </div>
    )
  }
}
