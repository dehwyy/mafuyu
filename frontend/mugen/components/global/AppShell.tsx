import Navbar from '@/components/global/Navbar'

export default function AppShell({ children }: { children: React.ReactNode }) {
  return (
    <div className="flex justify-center min-h-screen">
      <div className="flex flex-col w-full relative">
        <Navbar />
        <main className="flex-1 overflow-hidden">{children}</main>
      </div>
    </div>
  )
}
