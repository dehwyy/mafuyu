import { useEffect } from 'react'
import { Button, Modal, ModalBody, ModalContent, ModalFooter, ModalHeader, useDisclosure } from '@nextui-org/react'

interface ModalRemoveProps {
  children?: React.ReactNode

  title: string
  description: string

  isOpen: boolean
  setOpen: (v: boolean) => void
}

export function ModalRemove(props: ModalRemoveProps) {
  return (
    <Modal classNames={{ wrapper: 'z-[100000]' }} backdrop="opaque" isOpen={props.isOpen} onOpenChange={(v) => props.setOpen(v)}>
      <ModalContent>
        {(onClose) => (
          <>
            <ModalHeader>
              <div>
                <p className="text-lg">{props.title}</p>
                <p className="text-default-300 text-[12px]">{props.description}</p>
              </div>
            </ModalHeader>
            <ModalBody>{props.children}</ModalBody>
            <ModalFooter>
              <div className="flex gap-x-2">
                <Button onClick={onClose} variant="bordered">
                  Cancel
                </Button>
                <Button onClick={onClose} color="danger" variant="shadow">
                  Remove it please!
                </Button>
              </div>
            </ModalFooter>
          </>
        )}
      </ModalContent>
    </Modal>
  )
}
