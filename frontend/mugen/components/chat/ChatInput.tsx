import { useState } from 'react'
import { Input, Textarea } from '@nextui-org/input'

export default function ChatInput() {
  const [value, setValue] = useState('')
  return (
    <div className="max-w-full w-[600px] self-center mt-3">
      <Input
        value={value}
        onValueChange={setValue}
        variant="flat"
      />
    </div>
  )
}
