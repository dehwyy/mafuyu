import { useState } from 'react'
import {
  Button,
  CardBody,
  CardHeader,
  Divider,
  Input,
  Link,
  Modal,
  ModalBody,
  ModalContent,
  ModalFooter,
  ModalHeader,
  useDisclosure
} from '@nextui-org/react'

import { IconCheck } from '@/components/icons/Check'
import { IconPencil } from '@/components/icons/Pencil'
import { IconTrash } from '@/components/icons/Trash'
import { IconX } from '@/components/icons/X'
import { ButtonTooltiped } from '@/components/reusable/ButtonTooltiped'
import { CardPreseted } from '@/components/reusable/CardPreseted'
import { LinkSmooth } from '@/components/reusable/LinkSmooth'
import { ModalRemove } from '@/components/reusable/ModalRemove'
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
      <CardBody className="w-[400px]">
        <div className="flex flex-col gap-y-3">
          <Input variant="underlined" size="sm" defaultValue={props.channelName} spellCheck={false} />
          <div>
            <p className="text-center font-semibold mb-3">Breakpoints</p>
            <section className="flex flex-col gap-y-4 pb-3 max-h-[200px] overflow-y-auto">
              {breakpoints.map((item, i) => (
                <>
                  <Breakpoint key={i} {...item} />
                  {i < breakpoints.length - 1 && <Divider />}
                </>
              ))}
            </section>
          </div>
        </div>
      </CardBody>
    </CardPreseted>
  )
}

function Breakpoint(item: Breakpoint) {
  const [savedChannelName, setSavedChannelName] = useState(item.name)
  const [channelName, setChannelName] = useState(item.name)
  const [isNameEdit, setNameEdit] = useState(false)
  const [isRemoveModalOpen, setRemoveModalOpen] = useState(false)

  return (
    <>
      <div className="flex gap-x-4 text-sm items-center min-h-[32px]">
        {isNameEdit ? (
          <Input spellCheck={false} autoFocus value={channelName} onValueChange={setChannelName} size="sm" variant="underlined" />
        ) : (
          <p className="pl-1 pb-1.5">{channelName}</p>
        )}
        <div className="ml-auto flex gap-x-3 items-center">
          <LinkSmooth anchorId={item.startMessageId} className="font-semibold">
            Jump
          </LinkSmooth>
          <div className="flex">
            {isNameEdit ? (
              <>
                <Button
                  onClick={() => {
                    setNameEdit(false)
                    setChannelName(savedChannelName)
                  }}
                  size="sm"
                  className="bg-transparent hover:stroke-danger stroke-default-700"
                  isIconOnly
                  disableAnimation
                  disableRipple
                >
                  <IconX className="stroke-inherit transition-all" />
                </Button>
                <Button
                  onClick={() => {
                    setNameEdit(false)
                    setSavedChannelName(channelName)
                  }}
                  size="sm"
                  className="bg-transparent hover:stroke-success stroke-default-700"
                  isIconOnly
                  disableAnimation
                  disableRipple
                >
                  <IconCheck className="stroke-inherit transition-all" />
                </Button>
              </>
            ) : (
              <>
                <ButtonTooltiped
                  onClick={() => setNameEdit((v) => !v)}
                  isIconOnly
                  size="sm"
                  className="w-5 h-5 hover:stroke-secondary"
                  tooltip={{ content: 'Edit breakpoint', placement: 'top' }}
                >
                  <IconPencil className="stroke-2 w-5 h-5 stroke-inherit transition-all" />
                </ButtonTooltiped>
                <ButtonTooltiped
                  onPress={() => setRemoveModalOpen(true)}
                  isIconOnly
                  size="sm"
                  className="w-5 h-5 hover:stroke-danger"
                  tooltip={{ content: 'Delete breakpoint', placement: 'top' }}
                >
                  <IconTrash className="w-5 h-5 stroke-inherit transition-all" />
                </ButtonTooltiped>
              </>
            )}
          </div>
        </div>
      </div>
      <ModalRemove
        title="Remove breakpoint"
        description="Are you sure you want to remove this breakpoint?"
        isOpen={isRemoveModalOpen}
        setOpen={setRemoveModalOpen}
      />
    </>
  )
}
