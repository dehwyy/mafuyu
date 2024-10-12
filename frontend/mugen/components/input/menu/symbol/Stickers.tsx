import { Image } from '@nextui-org/react'

import { Dev } from '@/lib/const'

export const Stickers = () => {
  const stickers = new Array(24).fill(Dev.Sticker)

  return (
    <div className="grid grid-cols-4 gap-2 px-1 h-[200px] max-h-[200px] overflow-y-auto">
      {stickers.map((sticker) => (
        <Image
          className="cursor-pointer"
          key={sticker}
          alt="1"
          src={sticker}
        />
      ))}
    </div>
  )
}
