import { Divider } from '@nextui-org/react'

interface BottomPanelProps {
  children: React.ReactNode | React.ReactNode[]
  contentWidth?: string
  beforeContent?: React.ReactNode
}

export function BottomPanel({ children, contentWidth, beforeContent }: BottomPanelProps) {
  return (
    <div className="flex flex-col items-center w-full">
      {beforeContent}
      <div
        style={{ width: contentWidth ?? '100%' }}
        className="max-w-full self-center flex gap-x-2"
      >
        {children}
      </div>
    </div>
  )
}
