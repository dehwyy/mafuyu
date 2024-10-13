import { AppShell, Box, Container } from '$layout/essential'

import { CurrentServer } from './voice/CurrentServer'
import { NavigationPanel } from './voice/NavigationPanel'

export function VoiceLayout() {
  return (
    <AppShell withHeader>
      <Container width="250px">
        <Box h="64px">hello</Box>
        <Container
          grow
          horizontal
        >
          <Box scrollable>
            <NavigationPanel items={new Array(66).fill({})} />
          </Box>
          <Box scrollable>
            <CurrentServer />
          </Box>
        </Container>
        <Box h="64px">hello</Box>
      </Container>
      <Container grow>
        <Box h="64px">box top</Box>
        <Box
          scrollable
          h="50%"
          variant="unstyled"
          className="p-0"
        >
          scrollable down
        </Box>
        <Box grow>Fixed bottom</Box>
        <Box h="50px">Fixed bottom</Box>
      </Container>
    </AppShell>
  )
}
