import { useRef, useState } from 'react'
import { Input } from '@nextui-org/input'
import { Button } from '@nextui-org/react'
import { IconSend } from '$icons/Send'

import { MenuAttachment } from './menu/attach'
import MenuSymbol from './menu/symbol'

export function ChatInput() {
  const [value, setValue] = useState('')
  const ref = useRef<HTMLInputElement>(null)

  return (
    <>
      <MenuSymbol />
      <Input
        baseRef={ref}
        classNames={{
          inputWrapper: ['hover:dark:bg-default-100', 'shadow-md', 'shadow-default/50', 'min-h-none', 'relative']
        }}
        value={value}
        placeholder="Message"
        autoComplete="off"
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
    </>
  )
}
