import { Button } from '@nextui-org/button';
import { AppShell, Box, Container } from '$layout/essential';



import { Dev } from '@/lib/const';
import { IconPlus } from '../icons/Plus';
import { OverlaySettings } from './builder/aside';
import { ChatInput } from './chat';
import { CurrentServer } from './voice/CurrentServer';
import { NavigationPanel } from './voice/NavigationPanel';
import { UserCard } from './voice/UserCard'
import { ViewOverview } from './voice/ViewOverview'
import { ViewChat } from './voice/views/Chat';

export function VoiceLayout() {
  return (
    <AppShell withHeader>
      <Container width="300px">
        <Container
          h="64px"
          horizontal
          className="items-center"
        >
          <Box
            variant="unstyled"
            className="!py-0"
          >
            <Button
              isIconOnly
              color="secondary"
              variant="shadow"
              className="shadow-sm outline-none focus-visible:!outline-none h-[44px] w-[44px] rounded-[25px]"
            >
              <IconPlus className="stroke-gray-100" />
            </Button>
          </Box>
          <Box
            variant="unstyled"
            grow
          >
            <OverlaySettings />
          </Box>
        </Container>
        <Container
          grow
          horizontal
        >
          <Box scrollable>
            <NavigationPanel items={new Array(66).fill({})} />
          </Box>
          <Box
            scrollable
            grow
            className="!p-0"
          >
            <CurrentServer />
          </Box>
        </Container>
        <Box h="64px">
          <UserCard
            username="Waypo1nt"
            userImage={Dev.Img2}
            userStatus="online"
          />
        </Box>
      </Container>
      <Container grow>
        <Box
          h="48px"
          className="justify-center"
        >
          <ViewOverview />
        </Box>
        <Box
          scrollable
          variant="unstyled"
          className="p-0"
        >
          <ViewChat />
        </Box>
        <Box h="64px">
          <ChatInput />
        </Box>
      </Container>
      <Container width="200px">
        <Box h="64px">Statistics</Box>
        <Box h="64px">Input</Box>
        <Box scrollable>content</Box>
      </Container>
    </AppShell>
  )
}
