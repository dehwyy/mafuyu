import { useState } from 'react'
import { useHover } from '@custom-react-hooks/all'
import { Button, ButtonProps, Tooltip, TooltipProps } from '@nextui-org/react'
import clsx from 'clsx'

interface ButtonTooltipedProps extends ButtonProps {
  children: React.ReactNode
  tooltip: TooltipProps

  className?: string
}

export function ButtonTooltiped({ children, className, ...props }: ButtonTooltipedProps) {
  const [isTooltipOpen, setTooltipOpen] = useState(false)
  const { isHovered, ref } = useHover<HTMLButtonElement>()

  return (
    <Tooltip
      isDisabled={isTooltipOpen}
      placement={props.tooltip.placement || 'bottom'}
      delay={0}
      closeDelay={0}
      showArrow={props.tooltip.showArrow}
      content={props.tooltip.content}
    >
      <Button
        ref={ref}
        aria-expanded={props.isIconOnly}
        disableAnimation={props.isIconOnly}
        disableRipple={props.isIconOnly}
        className={clsx(
          props.isIconOnly
            ? 'bg-transparent hover:!opacity-100' + (isHovered || isTooltipOpen ? ' !stroke-default-800' : ' !stroke-default-500')
            : '',
          className
        )}
        {...props}
      >
        {children}
      </Button>
    </Tooltip>
  )
}
