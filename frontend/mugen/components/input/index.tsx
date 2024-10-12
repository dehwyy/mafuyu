import { useState } from 'react'
import { Input } from '@nextui-org/input'
import { Button, Divider } from '@nextui-org/react'

import { IconSend } from '../icons/Send'
import { MenuAttachment } from './menu/attach'
import MenuSymbol from './menu/symbol'

export default function ChatInput() {
  const [value, setValue] = useState('')
  return (
    <div className="flex flex-col items-center w-full">
      <Divider className="mb-1 bg-default-foreground/40 w-[640px] max-w-full" />
      <div className="max-w-full w-[600px] self-center flex gap-x-2">
        <MenuSymbol />
        <Input
          classNames={{
            inputWrapper: ['hover:dark:bg-default-100', 'shadow-md', 'shadow-default/50']
          }}
          value={value}
          onValueChange={setValue}
          variant="flat"
          endContent={<MenuAttachment />}
        />
        <Button
          isIconOnly
          variant="shadow"
          color="secondary"
        >
          <IconSend />
        </Button>
      </div>
    </div>
  )
}
