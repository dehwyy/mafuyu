import { useMemo } from 'react'
import { Button, Card, CardBody, ScrollShadow } from '@nextui-org/react'
import clsx from 'clsx'
import { useAtomValue } from 'jotai'

import { TransformTranslate } from '@/lib/const'
import { NavbarAtom } from '@/lib/store/global'

export default function ChatList() {
  const navbar = useAtomValue(NavbarAtom)
  const h = useMemo(() => {
    return 693 + +(navbar.isExpanded ? 0 : TransformTranslate.NavbarHide) + 'px'
  }, [navbar.isExpanded])

  return (
    <ScrollShadow
      style={{ maxHeight: h }}
      className="w-[200px] my-3 pr-1 transition-all"
    >
      <Card className={clsx('')}>
        <CardBody className="flex flex-col gap-y-2 !px-1 overflow-y-auto">
          {new Array(20).fill(0).map((_, i) => (
            <ChatListItem key={i} />
          ))}
        </CardBody>
      </Card>
    </ScrollShadow>
  )
}

function ChatListItem() {
  return (
    <Button
      className="w-full focus-visible:!outline-none min-h-[50px]"
      variant="light"
      radius="sm"
    >
      Chat's name
    </Button>
  )
}
