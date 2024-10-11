import { useState } from 'react'
import { Input, Textarea } from '@nextui-org/input'
import {
  Button,
  Divider,
  Dropdown,
  DropdownItem,
  DropdownMenu,
  DropdownSection,
  DropdownTrigger,
  Image,
  Tab,
  Tabs
} from '@nextui-org/react'

import { Dev } from '@/lib/const'
import { IconFile } from '../icons/File'
import { IconPaperclip } from '../icons/Paperclip'
import { IconPhoto } from '../icons/Photo'
import { IconSend } from '../icons/Send'
import { IconSticker } from '../icons/Sticker'

export default function ChatInput() {
  const [value, setValue] = useState('')
  return (
    <div className="flex flex-col items-center w-full">
      <Divider className="mb-2 bg-default-foreground/40 w-[640px] max-w-full" />
      <div className="max-w-full w-[600px] self-center flex gap-x-2">
        <Sticker />
        <Input
          classNames={{
            inputWrapper: ['hover:dark:bg-default-100', 'shadow-md', 'shadow-default/50']
          }}
          value={value}
          onValueChange={setValue}
          variant="flat"
          endContent={<Attachment />}
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

const Sticker = () => {
  const items = [
    {
      id: 'sticker',
      label: 'Stickers',
      content: () => {
        const stickers = new Array(24).fill(Dev.Sticker)

        return (
          <div className="grid grid-cols-4 gap-2 px-1 max-h-[200px] overflow-y-auto ">
            {stickers.map((sticker) => (
              <Image
                className="cursor-pointer "
                key={sticker}
                alt="1"
                src={sticker}
              />
            ))}
          </div>
        )
      }
    },
    {
      id: 'emoji',
      label: 'Emojis',
      content: () => {
        const emojis =
          '😀 😃 😄 😁 😆 😅 😂 🤣 😊 😇 🙂 🙃 😉 😌 😍 🥰 😘 😗 😙 😚 😋 😛 😝 😜 🤪 🤨 🧐 🤓 😎 🤩 🥳 😏 😒 😞 😔 😟 😕 🙁 😣 😖 😫 😩 🥺 😢 😭 😮 💨 😤 😠 😡 🤬 🤯 😳 🥵 🥶 😱 😨 😰 😥 😓 🤗 🤔 🤭 🤫 🤥 😶 😶 😑 😬 🙄 😯 😦 😧 😮 😲 🥱 😴 🤤 😪 😵 😵 🤐 🥴 🤢 🤮 🤧 😷 🤒 🤕 🤑 🤠 😈 👿 👹 👺 🤡 💩 👻 👽 👾 🤖 🎃 😺 😸 😹 😻 😼 😽 🙀 😿 😾'

        return (
          <div className="grid grid-cols-7 justify-between gap-y-3 px-1 max-h-[200px] overflow-y-auto">
            {emojis.split(' ').map((emoji) => (
              <Button
                key={emoji}
                isIconOnly
                disableAnimation
                className="bg-transparent"
              >
                <p className="text-3xl">{emoji}</p>
              </Button>
            ))}
          </div>
        )
      }
    }
  ]

  return (
    <Dropdown
      classNames={{
        content: 'p-0',
        base: 'left-[185px] bottom-1'
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

const Attachment = () => {
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
