import { OverlaySettings, UserBriefInformation } from '$layout/aside'
import { AppShell, Box, Container } from '$layout/essential'

import { ChatInput, ChatList, CurrentChatInformation, MessageList } from '@/components/@layout/chat'
import { Dev } from '@/lib/const'

export function ChatLayout() {
  return (
    <AppShell withHeader>
      <Container width="250px">
        <Box variant="unstyled">
          <OverlaySettings />
        </Box>
        <Box scrollable>
          <ChatList items={[]} />
        </Box>
      </Container>
      <Container grow>
        <Box h="64px">
          <CurrentChatInformation
            name="dehwyy"
            avatarSrc={Dev.Img}
          />
        </Box>
        <Box
          scrollable
          variant="unstyled"
          className="items-center"
        >
          <MessageList items={[]} />
        </Box>
        <Box
          w="600px"
          variant="unstyled"
          alignSelf="center"
        >
          <ChatInput />
        </Box>
      </Container>
    </AppShell>
  )
}
