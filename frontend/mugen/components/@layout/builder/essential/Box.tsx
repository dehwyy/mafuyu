import { useMemo } from 'react';
import { Card, CardBody, ScrollShadow } from '@nextui-org/react';
import clsx from 'clsx';



import { CardPreseted, Variant } from '@/components/reusable/CardPreseted';


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
  variant?: Variant
}

export function Box({ children, className, ...props }: BoxProps) {
  return (
    <Wrapper {...props}>
      <CardPreseted variant={props.variant} className="min-h-full">
        <CardBody className={clsx('flex flex-col gap-y-2 overflow-y-auto', className)}>{children}</CardBody>
      </CardPreseted>
    </Wrapper>
  )
}

function Wrapper(props: WrapperProps) {
  return props.scrollable ? (
    <ScrollShadow style={{ height: props.h ?? '100%' }} className={clsx('pr-1', props.grow && 'flex-1')}>
      {props.children}
    </ScrollShadow>
  ) : (
    <div
      style={{ height: props.h, minHeight: props.h, maxHeight: props.h, width: props.w, alignSelf: props.alignSelf }}
      className={clsx(props.grow && 'flex-1')}
    >
      {props.children}
    </div>
  )
}
