import { Button, Dropdown, DropdownItem, DropdownMenu, DropdownTrigger, Tab, Tabs } from '@nextui-org/react'

import { IconSticker } from '@/components/icons/Sticker'
import { Emojis } from './Emojis'
import { Stickers } from './Stickers'

export default function MenuSymbol() {
  const items = [
    {
      id: 'sticker',
      label: 'Stickers',
      content: Stickers
    },
    {
      id: 'emoji',
      label: 'Emojis',
      content: Emojis
    }
  ]

  return (
    <Dropdown
      classNames={{
        content: 'p-0',
        base: 'left-[184px] bottom-1'
      }}
    >
      <DropdownTrigger>
        <Button
          isIconOnly
          variant="shadow"
        >
          <IconSticker />
        </Button>
      </DropdownTrigger>
      <DropdownMenu
        closeOnSelect={false}
        classNames={{
          base: 'p-0'
        }}
      >
        <DropdownItem
          isReadOnly
          className="cursor-default !p-0"
          classNames={{}}
        >
          <Tabs
            variant="light"
            placement="bottom"
            items={items}
            className="w-[400px] flex justify-center"
          >
            {(item) => (
              <Tab
                key={item.id}
                title={item.label}
                className="w-[400px]"
              >
                <item.content />
              </Tab>
            )}
          </Tabs>
        </DropdownItem>
      </DropdownMenu>
    </Dropdown>
  )
}
