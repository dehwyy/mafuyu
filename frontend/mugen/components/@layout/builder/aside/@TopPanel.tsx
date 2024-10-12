interface TopPanelProps {
  children: React.ReactNode
}

const height = 48
export function TopPanel({ children }: TopPanelProps) {
  return <section style={{ height, minHeight: height, maxHeight: height }}>{children}</section>
}
