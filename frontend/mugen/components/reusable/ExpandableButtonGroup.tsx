import { useEffect, useMemo, useRef, useState } from 'react'
import { Button } from '@nextui-org/button'
import clsx from 'clsx'

interface ButtonGroupProps {
  items: {
    variant: string
    color: string
    content: string
  }[]
  itemHeight: number
  gap: number

  className?: string
}

export const ExpandableButtonGroup = ({ items: buttons, itemHeight, gap, className }: ButtonGroupProps) => {
  const [mainButton, ...items] = useMemo(() => buttons, [buttons])

  const expandButtonRef = useRef<HTMLButtonElement>(null)
  const containerRef = useRef<HTMLDivElement>(null)
  const [isExpanded, setIsExpanded] = useState(false)

  useEffect(() => {
    if (expandButtonRef.current && containerRef.current) {
      containerRef.current.style.maxHeight = (isExpanded ? itemHeight * (items.length + 1) + gap : itemHeight) + 'px'
    }
  }, [expandButtonRef.current, containerRef.current, isExpanded])

  return (
    <div
      ref={containerRef}
      className={clsx(className, 'transition-all duration-300 ease-in-out')}
    >
      <div
        style={{ gap: `${gap}px 0` }}
        className="overflow-hidden flex flex-col"
      >
        <Button
          onClick={() => setIsExpanded((v) => !v)}
          ref={expandButtonRef}
          style={{ minHeight: itemHeight, maxHeight: itemHeight }}
          variant={mainButton.variant as any}
          color={mainButton.color as any}
        >
          {mainButton.content}
        </Button>
        {items.map((v, i) => (
          <Button
            key={i}
            style={{ minHeight: itemHeight, maxHeight: itemHeight }}
            variant={v.variant as any}
            color={v.color as any}
            className={clsx(isExpanded ? 'opacity-100 visible' : 'opacity-0 invisible', 'transition-all duration-500 ease-in-out')}
          >
            {v.content}
          </Button>
        ))}
      </div>
    </div>
  )
}
