import { Button } from '@nextui-org/button'
import { AsideTopPanel, UserBriefInformation } from '$layout/aside'
import { Content, ContentBottomPanel, ContentScrollable, ContentTopPanel } from '$layout/content'
import { AppShell } from '$layout/essential'

import { Box } from './builder/essential/Box'
import { Container } from './builder/essential/Container'
import { CurrentServer } from './voice/CurrentServer'
import { NavigationPanel } from './voice/NavigationPanel'

export function Voice2Layout() {
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
