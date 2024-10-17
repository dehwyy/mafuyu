import { useState } from 'react'
import { useHover } from '@custom-react-hooks/all'
import { Button, Popover, PopoverContent, PopoverTrigger, Tooltip } from '@nextui-org/react'
import clsx from 'clsx'

import { IconProps } from '@/types'

interface ChatOverviewActionActionProps {
  children: React.ReactNode // viewComponent
  triggerIcon: (props: IconProps) => JSX.Element
  text: string
}

export function ChatOverviewActionTrigger(props: ChatOverviewActionActionProps) {
  const [isPopoverOpen, setPopoverOpen] = useState(false)
  const [isPopoverMounted, setPopoverMounted] = useState(false)

  const { isHovered, ref } = useHover<HTMLButtonElement>()

  return (
    <Popover
      isOpen={isPopoverOpen}
      onOpenChange={(state) => {
        setPopoverOpen(state)

        // reduces layers collapse
        setPopoverMounted(false)
        queueMicrotask(() => setPopoverMounted(true))
      }}
      placement="bottom-end"
    >
      <Tooltip isDisabled={isPopoverOpen} content={props.text} placement="bottom" showArrow closeDelay={0} delay={0}>
        <div>
          <PopoverTrigger>
            <Button
              // todo
              aria-expanded={false}
              ref={ref}
              className={clsx(
                'bg-transparent  hover:!opacity-100',
                isHovered || isPopoverOpen ? '!stroke-default-800' : '!stroke-default-500'
              )}
              disableAnimation
              disableRipple
              isIconOnly
            >
              <props.triggerIcon className="stroke-2 transition-all" />
            </Button>
          </PopoverTrigger>
        </div>
      </Tooltip>
      <PopoverContent className={clsx(isPopoverMounted ? 'bg-transparent' : 'bg-background', ' shadow-none !p-0 transition-all delay-300')}>
        {props.children}
      </PopoverContent>
    </Popover>
  )
}
