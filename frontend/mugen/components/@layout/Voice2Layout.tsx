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
    </AppShell>
    // <AppShell withHeader>
    //   <Aside>
    //     <AsideTopPanel>
    //       <div className="flex gap-x-3 h-full">
    //         <Button
    //           className="h-full"
    //           variant="shadow"
    //           color="secondary"
    //         >
    //           Change layout
    //         </Button>
    //         <Button
    //           className="h-full"
    //           variant="shadow"
    //           color="secondary"
    //         >
    //           Manage overlays
    //         </Button>
    //       </div>
    //     </AsideTopPanel>
    //     <div className="flex flex-row gap-x-[4px] max-h-full min-h-full">
    //       <AsideScrollable>
    //         <NavigationPanel items={new Array(66).fill({})} />
    //       </AsideScrollable>
    //       <div className="flex-1">
    //         <AsideScrollable>
    //           <CurrentServer />
    //         </AsideScrollable>
    //       </div>
    //     </div>
    //   </Aside>
    //   <Content>
    //     <ContentTopPanel>some</ContentTopPanel>
    //     <ContentScrollable>some</ContentScrollable>
    //     <ContentBottomPanel>some</ContentBottomPanel>
    //   </Content>
    // </AppShell>
  )
}
