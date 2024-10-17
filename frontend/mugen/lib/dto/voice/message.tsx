import { Image } from '@nextui-org/react'

import { Buildable } from '../global'

export interface MessageProps {
  component: Buildable
	messageId: string
  senderUsername: string
  senderImage?: string
  time: string
}

export type MessageComponentType = 'text' | 'image' | 'sticker'
interface Size {
  w: number
  h: number
}

export class MessageText implements Buildable {
  constructor(private text: string) {}

  build() {
    return <p className="text-[15px] font-normal text-default-700">{this.text}</p>
  }
}

export class MessageImage implements Buildable {
  private maxHeight = 200

  constructor(
    private src: string,
    private alt: string,
    private size?: Partial<Size>
  ) {}

  build() {
    return (
      <Image
        src={this.src}
        alt={this.alt}
        style={{ width: this.size?.w, height: this.size?.h, maxWidth: '100%', maxHeight: this.maxHeight }}
      />
    )
  }
}
