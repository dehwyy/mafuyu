import { Button } from '@nextui-org/button';
import { Input } from '@nextui-org/input';
import { Avatar, User } from '@nextui-org/react'
import { AppShell, Box, Container } from '$layout/essential'

import { Dev } from '@/lib/const'
import { IconPlus } from '../icons/Plus'
import { OverlaySettings } from './builder/aside'
import { ChatInput } from './chat'
import { CurrentServer } from './voice/CurrentServer'
import { NavigationPanel } from './voice/NavigationPanel'
import OnlineStats from './voice/stats/OnlineStats'
import { UserCard } from './voice/UserCard'
import { ViewOverview } from './voice/ViewOverview'
import { ViewChat } from './voice/views/Chat'

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
        <Box
          variant="unstyled"
          className="!py-1 -mb-2"
        >
          <span className="text-lg">Online</span>
        </Box>
        <Box
          h="64px"
          className="!p-0"
        >
          <OnlineStats
            width={200}
            height={64}
            my={24}
          />
        </Box>
        <Box
          h="64px"
          variant="unstyled"
        >
          <Input
            variant="underlined"
            size="sm"
            label="Search"
          />
        </Box>
        <Box
          scrollable
          className="flex flex-col gap-y-4"
        >
          {new Array(4).fill({}).map((_, index) => (
            <div className="flex gap-x-3 items-center h-[44px] py-[1px]">
              <div>
                <Avatar
                  className="h-8 w-8"
                  src={Dev.Img}
                />
              </div>
              <div className="w-2/3">
                <p className="text-[16px] leading-5 text-violet-400">Waypo1nt</p>
                <p className="ellipsis text-[12px] leading-3 text-default-400 font-medium">
                  Playing Counter-Strike 21231adsasadsdsadsa1211313
                </p>
              </div>
            </div>
          ))}
        </Box>
      </Container>
    </AppShell>
  )
}
