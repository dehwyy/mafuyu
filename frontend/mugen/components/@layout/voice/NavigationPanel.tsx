import { useMemo, useState } from 'react'
import { useHover } from '@custom-react-hooks/all'
import { Avatar, Tooltip } from '@nextui-org/react'

import { Dev } from '@/lib/const'

interface NavigationPanelProps {
  items: {}[]
}

export function NavigationPanel({ items }: NavigationPanelProps) {
  const [selected, setSelected] = useState<number | null>(null)

  return (
    <nav className="h-full flex flex-col gap-y-4">
      {items.map((item, i) => (
        <NavigationPanelItem
          key={i}
          isSelected={selected === i}
          setSelected={() => setSelected(i)}
        />
      ))}
    </nav>
  )
}

const NavigationPanelItem = ({ isSelected, setSelected }: { isSelected: boolean; setSelected: () => void }) => {
  const [isOpen, setIsOpen] = useState(false)

  return (
    <div className="relative flex gap-x-4">
      <Bar
        isHover={isOpen}
        isSelected={isSelected}
      />
      <Tooltip
        placement="right"
        content="server"
        closeDelay={0}
        delay={0}
        onOpenChange={setIsOpen}
      >
        <Avatar
          style={{ width: 44, height: 44, borderRadius: isOpen || isSelected ? '15px' : '25px' }}
          onClick={() => setSelected()}
          className="transition-all duration-300 ease-in-out cursor-pointer"
          src={Dev.Img}
        />
      </Tooltip>
    </div>
  )
}

const Bar = ({ isHover, isSelected }: { isHover: boolean; isSelected: boolean }) => {
  const isAnyEvent = useMemo(() => {
    return isHover || isSelected
  }, [isHover, isSelected])

  const h = useMemo(() => {
    const h = isSelected ? 80 : 60
    return h + '%'
  }, [isHover, isSelected])

  return (
    <div
      style={{ opacity: isAnyEvent ? 1 : 0, height: h }}
      className="transition-all rounded-sm bg-white w-[6px] absolute -left-3.5 top-1/2 -translate-y-1/2"
    />
  )
}
