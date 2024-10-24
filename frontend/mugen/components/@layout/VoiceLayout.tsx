import { Button } from '@nextui-org/button';
import { Input } from '@nextui-org/input';
import { Avatar, user, User } from '@nextui-org/react';
import { AppShell, Box, Container } from '$layout/essential';



import { Dev } from '@/lib/const';
import { IconPlus } from '../icons/Plus';
import { OverlaySettings } from './builder/aside';
import { ChatInput } from './chat';
import { CreateServerButton } from './voice/CreateServerButton';
import { CurrentServer } from './voice/CurrentServer';
import { NavigationPanel } from './voice/NavigationPanel';
import { Category } from './voice/raside/Category';
import OnlineStats from './voice/stats/OnlineStats';
import { UserPanel } from './voice/UserPanel'
import { ViewOverview } from './voice/ViewOverview'
import { ViewChat } from './voice/views/chat/Chat'

export function VoiceLayout() {
  return (
    <AppShell withHeader>
      {/* L Aside  */}
      <Container width="300px">
        <Container h="64px" horizontal className="items-center">
          <Box className="!py-0">
            <CreateServerButton />
          </Box>
          <Box grow>
            <OverlaySettings />
          </Box>
        </Container>
        <Container grow horizontal>
          <Box scrollable variant="gradient">
            <NavigationPanel items={new Array(66).fill({})} />
          </Box>
          <Box scrollable grow className="!p-0" variant="gradient">
            <CurrentServer />
          </Box>
        </Container>
        <Box h="64px" variant="gradient">
          <UserPanel username="Waypo1nt" userImage={Dev.Img2} userStatus="online" />
        </Box>
      </Container>

      {/* Main  */}
      <Container grow>
        <Box h="56px" variant="gradient" className="justify-center" w="98%">
          <ViewOverview />
        </Box>
        <Box scrollable className="p-0">
          <ViewChat />
        </Box>
        <Box h="64px">
          <ChatInput />
        </Box>
      </Container>

      {/* R Aside  */}
      <Container width="200px" className="select-none">
        <Box className="!py-1 -mb-2 flex-row items-center justify-between">
          <span className="text-lg">Online</span>
          <span className="text-sm text-default-500">(Last 12 hours)</span>
        </Box>
        <Box h="64px" className="!p-0" variant="gradient">
          <OnlineStats width={200} height={64} my={24} />
        </Box>
        <Box h="64px">
          <Input size="sm" variant="underlined" label="Search" />
        </Box>
        <Box variant="gradient" scrollable className="flex flex-col gap-y-4">
          {new Array(12).fill({}).map((_, index) => (
            <Category
              key={index}
              categoryName={`Category ${index}`}
              items={new Array(4).fill({ userImage: Dev.Img, username: `User ${index}`, activity: `Activity ${index}` })}
            />
          ))}
        </Box>
      </Container>
    </AppShell>
  )
}
