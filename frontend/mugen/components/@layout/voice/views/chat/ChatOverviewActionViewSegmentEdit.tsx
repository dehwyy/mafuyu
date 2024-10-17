import { CardBody, CardHeader, Input, Link } from '@nextui-org/react'

import { IconPencil } from '@/components/icons/Pencil'
import { IconTrash } from '@/components/icons/Trash'
import { ButtonTooltiped } from '@/components/reusable/ButtonTooltiped'
import { CardPreseted } from '@/components/reusable/CardPreseted'
import { LinkSmooth } from '@/components/reusable/LinkSmooth'
import { Dev } from '@/lib/const'

interface Breakpoint {
  name: string
  startMessageId: string
  breakpoints: string[]
}

const breakpoints: Breakpoint[] = [
  { name: 'Tanjoubi no koto', startMessageId: Dev.MessageId1, breakpoints: [] },
  { name: 'Shiawase ni tsuite', startMessageId: Dev.MessageId2, breakpoints: [] }
]

interface ChatOverviewActionViewSegmentEditProps {
  channelName: string
}

export function ChatOverviewActionViewSegmentEdit(props: ChatOverviewActionViewSegmentEditProps) {
  return (
    <CardPreseted variant="gradient">
      <CardHeader>
        <div className="flex gap-x-2 items-center">
          <IconPencil className="stroke-2" />
          <p className="text-lg">Edit channel</p>
        </div>
      </CardHeader>
      <CardBody className="w-[350px]">
        <div className="flex flex-col gap-y-3">
          <Input variant="underlined" size="sm" defaultValue={props.channelName} spellCheck={false} />
          <div>
            <p className="text-center font-semibold mb-3">Breakpoints</p>
            <section className="flex flex-col gap-y-2">
              {breakpoints.map((item, i) => (
                <article className="flex flex-col gap-y-1">
                  <div key={i} className="flex gap-x-2 text-sm">
                    <p>{item.name}</p>
                    <div className="ml-auto flex gap-x-3 items-center">
                      <LinkSmooth anchorId={item.startMessageId} className="">
                        Go to
                      </LinkSmooth>
                      <div className="flex">
                        <ButtonTooltiped
                          isIconOnly
                          size="sm"
                          className="w-5 h-5"
                          tooltip={{ content: 'Edit breakpoint', placement: 'left' }}
                        >
                          <IconPencil className="stroke-2 w-5 h-5" />
                        </ButtonTooltiped>
                        <ButtonTooltiped
                          isIconOnly
                          size="sm"
                          className="w-5 h-5"
                          tooltip={{ content: 'Delete breakpoint', placement: 'right' }}
                        >
                          <IconTrash className="w-5 h-5" />
                        </ButtonTooltiped>
                      </div>
                    </div>
                  </div>
                </article>
              ))}
            </section>
          </div>
        </div>
      </CardBody>
    </CardPreseted>
  )
}
