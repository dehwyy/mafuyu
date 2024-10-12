import { Divider } from '@nextui-org/react'

interface BottomPanelProps {
  children: React.ReactNode | React.ReactNode[]
}

export function BottomPanel({ children }: BottomPanelProps) {
  return (
    <div className="flex flex-col items-center w-full">
      <Divider className="mb-2 bg-default-foreground/40 w-[640px] max-w-full" />
      <div className="max-w-full w-[600px] self-center flex gap-x-2">{children}</div>
    </div>
  )
}
