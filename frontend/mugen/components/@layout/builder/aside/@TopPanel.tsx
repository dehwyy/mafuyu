interface TopPanelProps {
  children: React.ReactNode
}

export function TopPanel({ children }: TopPanelProps) {
  return <section className="min-h-[64px] max-h-[64px] h-[64px]">{children}</section>
}
