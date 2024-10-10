import AppShell from '@/components/global/AppShell'
import { LayoutProps } from '@/types'

export default function $Layout({ children }: LayoutProps) {
  return <AppShell>{children}</AppShell>
}
