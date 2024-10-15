import { Button } from '@nextui-org/button'

import { IconPlus } from '@/components/icons/Plus'

interface CreateServerButtonProps {}

export function CreateServerButton(props: CreateServerButtonProps) {
  return (
    <Button
      isIconOnly
      color="secondary"
      variant="shadow"
      className="shadow-sm outline-none focus-visible:!outline-none h-[44px] w-[44px] rounded-[25px]"
    >
      <IconPlus className="stroke-gray-100" />
    </Button>
  )
}
