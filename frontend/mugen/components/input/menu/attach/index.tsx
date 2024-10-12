import { Button, Dropdown, DropdownItem, DropdownMenu, DropdownTrigger } from '@nextui-org/react'

import { IconFile } from '@/components/icons/File'
import { IconPaperclip } from '@/components/icons/Paperclip'
import { IconPhoto } from '@/components/icons/Photo'

export const MenuAttachment = () => {
  const items = [
    { placeholder: 'Photo or video', icon: IconPhoto },
    { placeholder: 'File', icon: IconFile }
  ]
  return (
    <Dropdown>
      <DropdownTrigger>
        <Button
          isIconOnly
          className="bg-transparent"
        >
          <IconPaperclip />
        </Button>
      </DropdownTrigger>
      <DropdownMenu>
        {items.map((item) => (
          <DropdownItem
            key={item.placeholder}
            startContent={<item.icon />}
          >
            {item.placeholder}
          </DropdownItem>
        ))}
      </DropdownMenu>
    </Dropdown>
  )
}
